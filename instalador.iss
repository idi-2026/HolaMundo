[Setup]
AppName=HolaMundo
AppVersion=1.0.0
DefaultDirName={autopf}\HolaMundo
DefaultGroupName=HolaMundo
OutputDir=instalador_salida
OutputBaseFilename=HolaMundo_Setup
Compression=lzma
SolidCompression=yes
ArchitecturesInstallIn64BitMode=x64

[Files]
Source: "target\release\holamundo.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\HolaMundo"; Filename: "{app}\holamundo.exe"
Name: "{autodesktop}\HolaMundo"; Filename: "{app}\holamundo.exe"

[Run]
Filename: "{app}\holamundo.exe"; Description: "Ejecutar HolaMundo"; Flags: nowait postinstall skipifsilent
