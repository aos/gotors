#[derive(Debug)]
pub(crate) enum Shell {
    Bash,
    Fish,
    Zsh,
}

impl std::fmt::Display for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Bash => {
                write!(
                    f,
                    "{}",
                    r##"# To make the binary work, add the following lines of code
# to your ~/.bash_profile or ~/.bash_rc
#
# eval "$(goto --init)"
#
# It will autogenerate this text to make the magic happen.
g() {
  local dir="$(goto $@)"
  test -d "$dir" && cd "$dir" || echo "$dir"
}"##
                )
            }
            _ => todo!(),
        }
    }
}
