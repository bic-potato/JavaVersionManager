﻿; Script generated by the Inno Setup Script Wizard.
; SEE THE DOCUMENTATION FOR DETAILS ON CREATING INNO SETUP SCRIPT FILES!

#define MyAppName "Java Version Manager"
#define MyAppVersion "0.3.0"
#define MyAppPublisher "Bic Potato"
#define MyAppURL "https://github.com/bic-potato/JavaVersionManager"
#define MyAppExeName "jvmain.exe"

[Setup]
; NOTE: The value of AppId uniquely identifies this application. Do not use the same AppId value in installers for other applications.
; (To generate a new GUID, click Tools | Generate GUID inside the IDE.)
AppId={{8D86B7FB-6F06-4B04-8C70-3F8F74F8FD14}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
;AppVerName={#MyAppName} {#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={localappdata}\{#MyAppName}
;DefaultGroupName={#MyAppName}
LicenseFile=C:\Users\m1333\Documents\Programming\JavaVersionManager\LICENSE
; Uncomment the following line to run in non administrative install mode (install for current user only.)
;PrivilegesRequired=lowest
OutputDir=C:\Users\m1333\Documents\Programming\JavaVersionManager\target\release
OutputBaseFilename=JavaVersionManager_v0.3.0_x64
Compression=lzma
SolidCompression=yes
WizardStyle=modern
ChangesEnvironment=yes

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "SimplifiedChinese"; MessagesFile: "compiler:\Languages\ChineseSimplified.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "C:\Users\m1333\Documents\Programming\JavaVersionManager\target\release\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion
Source: "C:\Users\m1333\Documents\Programming\JavaVersionManager\jvman.cmd"; DestDir: "{app}"; Flags: ignoreversion
Source: "C:\Users\m1333\Documents\Programming\JavaVersionManager\jvman.ps1"; DestDir: "{app}"; Flags: ignoreversion
Source: "C:\Users\m1333\Documents\Programming\JavaVersionManager\versions.toml"; DestDir: "{app}"; Flags: onlyifdoesntexist
; NOTE: Don't use "Flags: ignoreversion" on any shared system files

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon

;[Run]
;Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent


[dirs]
Name:"{app}\java"
Name:"{app}\temp"

[UninstallDelete]
Type: filesandordirs; Name: "{app}\java"
Type: filesandordirs; Name: "{app}\temp"

[Registry]
Root: HKLM; Subkey:"SYSTEM\CurrentControlSet\Control\Session Manager\Environment";ValueType: expandsz; ValueName:"Path"; ValueData:"{app}/OpenJDK/bin;{app};{olddata}";Check: NeedsAddPath('{app}')


[Code]

const
  EnvironmentKey = 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment';
function NeedsAddPath(Param: string): boolean;
var
  OrigPath: string;
begin
  if not RegQueryStringValue(HKEY_LOCAL_MACHINE,
    EnvironmentKey,
    'Path', OrigPath)
  then begin
    Result := True;
    exit;
  end;
  { look for the path with leading and trailing semicolon }
  { Pos() returns 0 if not found }
  Result := Pos(';' + Param + ';', ';' + OrigPath + ';') = 0;
end;
procedure RemovePath(Path: string);
var
  Paths: string;
  P: Integer;
begin
  if not RegQueryStringValue(HKLM, EnvironmentKey, 'Path', Paths) then
  begin
    Log('PATH not found');
  end
    else
  begin
    Log(Format('PATH is [%s]', [Paths]));

    P := Pos(';' + Uppercase(Path) + ';', ';' + Uppercase(Paths) + ';');
    if P = 0 then
    begin
      Log(Format('Path [%s] not found in PATH', [Path]));
    end
      else
    begin
      if P > 1 then P := P - 1;
      Delete(Paths, P, Length(Path) + 1);
      Log(Format('Path [%s] removed from PATH => [%s]', [Path, Paths]));

      if RegWriteStringValue(HKLM, EnvironmentKey, 'Path', Paths) then
      begin
        Log('PATH written');
      end
        else
      begin
        Log('Error writing PATH');
      end;
    end;
  end;
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
  if CurUninstallStep = usUninstall then
  begin
    RemovePath(ExpandConstant('{app}'));
    RemovePath(ExpandConstant('{app}/OpenJDK/bin'));
  end;
end;


