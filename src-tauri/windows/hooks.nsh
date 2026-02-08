!macro NSIS_HOOK_POSTUNINSTALL
  ; Clean up AppData on uninstall
  RMDir /r "$LOCALAPPDATA\Constellation"
  RMDir /r "$APPDATA\Constellation"
!macroend
