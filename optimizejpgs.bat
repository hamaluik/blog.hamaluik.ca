@echo off
call :treeProcess
goto :eof

:treeProcess
for %%f in (*.jpg) do (
    echo "Stripping %%f..."
    magick convert "%%f" -sampling-factor 4:2:0 -strip -quality 85 "%%~nf.jpg"
)
for /D %%d in (*) do (
    cd %%d
    call :treeProcess
    cd ..
)
exit /b