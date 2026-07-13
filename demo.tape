# demo.tape — render with: vhs demo.tape
Output demo.gif
Require diorama

Set Shell "bash"
Set FontFamily "JetBrainsMono Nerd Font Mono"
Set FontSize 14
Set Width 1200
Set Height 750
Set Padding 20
Set Theme "Catppuccin Mocha"

# Launch hidden so the GIF opens already in-app
Hide
Type "diorama --offline"
Enter
Sleep 1s
Show

# Grid overview: scroll down and back
Sleep 1.5s
Down@400ms 3
Sleep 1s
Up@400ms 3
Sleep 800ms

# Fullscreen, then cycle through panels
Tab
Sleep 1.5s
Right@700ms 5
Sleep 1s

# Scroll within a panel
Down@250ms 6
Sleep 1s

# Back to grid and quit
Escape
Sleep 800ms
Type "q"
