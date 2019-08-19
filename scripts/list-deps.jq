# The Syntax of jq is very functional

# Parse a package structure formatted by the `cargo metadata` command into a package spec.
# https://doc.rust-lang.org/cargo/commands/cargo-metadata.htm
# https://doc.rust-lang.org/cargo/reference/pkgid-spec.html
def package_spec:
    if (.source != null) then
        "\(.source)#\(.name):\(.version)"
    else
        "\(.name):\(.version)"
    end;

# Parse a workspace member formatted "my-package 0.1.0 (path+file:///path/to/my-package)" into "my-package"
# https://doc.rust-lang.org/cargo/commands/cargo-metadata.html
def parse_package_id: .
    | split(" ")
    | {
        name: .[0],
        version: .[1],
        source: .[2] | split("?")[0] | split("#")[0] | split("+")[1] | split(")")[0],
    };

def get_packages_without_workspace_members:
    # Build a list of workspace members, so we can skip them later.
    (.workspace_members | map(parse_package_id)) as $workspace_members |
    (.resolve.nodes | map(.id | parse_package_id)) as $packages |
    # Get a list of all packages, exclude any in the workspace member list, build their spec,
    # then print them like so `--package spec1 --package spec2...`
    $packages | map(. as $package | select($workspace_members | map($package.name != .name) | all));

# Build the arg string for `cargo build` to specify to build this package.
# Check `cargo build --help` for docs
def build_package_args: .
    | get_packages_without_workspace_members
    | map(package_spec)
    | map("--package \"\(.)\"");


def build_exclude_args: .workspace_members
    | map(parse_package_id)
    | map(" --exclude \"\(.)\"");

. | build_package_args[]
# . | build_exclude_args[]


#  |
# (. | build_exclude_args) as $exclude_args |
# "\($package_args) \($exclude_args)"
