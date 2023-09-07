@echo off

REM Check if an input video file is provided as an argument
if "%~1"=="" (
    echo Usage: %~nx0 ^<input_video^>
    exit /b 1
)

REM Input video file provided as argument
set "input_video=%~1"

echo Input video: %input_video%

REM Create the 'video' directory or clear it if already exists
if exist "video" (
    
    echo 'video' directory already exists. Because batch hates me, I cant ask you if i should clear it for you so.... pls delete it and try again, ty
    exit /b 1
) else (
    mkdir "video"
)

REM Extract frames from the input video
@echo on
ffmpeg -i "%input_video%" "video\apple-%%05d.png"
@echo off
REM Extract audio from the input video and save as 'audio.mp3'
@echo on
ffmpeg -i "%input_video%" -vn -acodec libmp3lame "audio.mp3"
@echo off
echo Frames extracted to 'video' directory, and audio saved as 'audio.mp3'

