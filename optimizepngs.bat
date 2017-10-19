@echo off
call :treeProcess
goto :eof

:treeProcess
for %%f in (*.png) do (
    echo "Stripping %%f..."
    magick convert "%%f" -strip "%%~nf.png"
)
for /D %%d in (*) do (
    cd %%d
    call :treeProcess
    cd ..
)
exit /b