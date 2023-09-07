#!/bin/bash

# Check if an input video file is provided as an argument
if [ $# -ne 1 ]; then
    echo "Usage: $0 <input_video>"
    exit 1
fi

# Input video file provided as argument
input_video="$1"

# Create the 'video' directory or clear it if already exists
if [ -d "video" ]; then
    echo "'video' directory already exists. Do you want to remove its contents? (y/n)"
    read confirmation

    if [ "$confirmation" == "y" ]; then
        rm -r video/*
    else
        exit 0
    fi
else
    mkdir video
fi

# Extract frames from the input video
ffmpeg -i "$input_video" "video/apple-%05d.png"

# Extract audio from the input video and save as 'audio.mp3'
ffmpeg -i "$input_video" -vn -acodec libmp3lame "audio.mp3"

echo "Frames extracted to 'video' directory, and audio saved as 'audio.mp3'"
