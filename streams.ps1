$baseDir = "C:\eframe\eframe2\assets\vidos"
$distDir = "C:\eframe\eframe2\dist\streams"
$logFile = "C:\eframe\eframe2\streams.log"
$ffmpegPath = "C:\eframe\eframe2\ffmpeg\bin\ffmpeg.exe" # Укажите свой путь к ffmpeg.exe

function Log {
    param($msg)
    $ts = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $line = "$ts $msg"
    Write-Host $line
    Add-Content -Path $logFile -Value $line
}

$streams = Get-ChildItem -Path $baseDir -Directory

function Start-StreamJob {
    param($streamName, $streamPath, $targetDir, $logFile, $ffmpegPath)

    Start-Job -ArgumentList $streamName, $streamPath, $targetDir, $logFile, $ffmpegPath -ScriptBlock {
        param($streamName, $streamPath, $targetDir, $logFile, $ffmpegPath)
        function Log {
            param($msg)
            $ts = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
            $line = "$ts $msg"
            Write-Host $line
            Add-Content -Path $logFile -Value $line
        }
        while ($true) {
            $files = Get-ChildItem -Path $streamPath -File | Where-Object { $_.Extension -match '\.(mp4|mkv|avi)$' } | Sort-Object Name
            if ($files.Count -eq 0) {
                Start-Sleep -Seconds 5
                continue
            }
            foreach ($file in $files) {
                $playlist = Join-Path $targetDir "playlist.m3u8"
                Log "[$streamName] Транслируем $($file.Name) в $playlist"
                try {
                    & $ffmpegPath -y -re -i $file.FullName -c:v libx264 -c:a aac -f hls -hls_time 4 -hls_list_size 5 -hls_flags delete_segments $playlist 2>&1 | ForEach-Object { Log $_ }
                    # Remove-Item $file.FullName
                } catch {
                    Log "[$streamName] Ошибка запуска ffmpeg: $_"
                }
            }
        }
    }
}

foreach ($stream in $streams) {
    $streamName = $stream.Name
    $streamPath = $stream.FullName
    $targetDir = Join-Path $distDir $streamName
    mkdir $targetDir -Force | Out-Null

    Log "Запуск потока: $streamName"
    Start-StreamJob $streamName $streamPath $targetDir $logFile $ffmpegPath
}

Log "Все потоки запущены как фоновые задачи. Для просмотра статуса используйте Get-Job."