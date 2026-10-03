_wlr_themes() {
  # Split on newlines only, so names with spaces stay whole
  local -a themes
  themes=(${(f)"$(wlr list --plain 2>/dev/null)"})

  compadd -a themes
}
