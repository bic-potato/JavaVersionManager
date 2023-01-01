@ECHO OFF

IF "%JVS_HOME%"=="" SET JVS_HOME=%~dp0

IF "%JVS_LEVEL%"=="DEBUG" (
    SET ExecutePath="%JVS_HOME%target\debug\jvmain.exe"
    ) ELSE (
        SET ExecutePath="%JVS_HOME%jvmain.exe" 
        )

SET /A JVS_POSTSCRIPT=%RANDOM% * 32568 + %RANDOM%
SET JVS_POSTSCRIPT=%JVS_HOME%nvs_tmp_%JVS_POSTSCRIPT%.cmd
SET JVS_EXEC_SHELL=CMD

 %ExecutePath% %*

IF NOT EXIST "%JVS_POSTSCRIPT%" GOTO :CLEANUP
CALL "%JVS_POSTSCRIPT%"
DEL "%JVS_POSTSCRIPT%"

:CLEANUP
SET JVS_POSTSCRIPT=

EXIT /B 