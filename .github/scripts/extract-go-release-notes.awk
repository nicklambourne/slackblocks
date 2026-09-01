$0 ~ "^## \\[" version "\\]" { capture = 1; next }
capture && /^## \[/ { exit }
capture && !started && /^[[:space:]]*$/ { next }
capture { print; started = 1 }
