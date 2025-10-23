//! Audio playback using rodio
//!
//! Provides simple audio playback for music and sound effects.

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

/// Audio source that can be played
pub struct AudioSource {
    data: Arc<Vec<u8>>,
}

impl AudioSource {
    /// Load an audio file from disk
    ///
    /// Supports: WAV, MP3, OGG, FLAC
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let data = std::fs::read(path.as_ref())
            .map_err(|e| format!("Failed to read audio file: {}", e))?;
        
        log::info!("Loaded audio file: {:?}", path.as_ref());
        Ok(Self {
            data: Arc::new(data),
        })
    }

    /// Create a decoder for this audio source
    fn decoder(&self) -> Result<Decoder<BufReader<std::io::Cursor<Vec<u8>>>>, String> {
        let cursor = std::io::Cursor::new(self.data.as_ref().clone());
        let buf_reader = BufReader::new(cursor);
        Decoder::new(buf_reader).map_err(|e| format!("Failed to decode audio: {}", e))
    }
}

/// Manages audio playback
pub struct AudioManager {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    music_sink: Option<Sink>,
    sfx_sinks: Vec<Sink>,
    master_volume: f32,
    music_volume: f32,
    sfx_volume: f32,
}

impl AudioManager {
    /// Create a new audio manager
    pub fn new() -> Result<Self, String> {
        let (stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| format!("Failed to create audio output stream: {}", e))?;

        log::info!("Audio system initialized");

        Ok(Self {
            _stream: stream,
            stream_handle,
            music_sink: None,
            sfx_sinks: Vec::new(),
            master_volume: 1.0,
            music_volume: 0.8,
            sfx_volume: 1.0,
        })
    }

    /// Play a sound effect
    pub fn play_sfx(&mut self, source: &AudioSource) -> Result<(), String> {
        let sink = Sink::try_new(&self.stream_handle)
            .map_err(|e| format!("Failed to create sink: {}", e))?;

        let decoder = source.decoder()?;
        sink.set_volume(self.master_volume * self.sfx_volume);
        sink.append(decoder);
        sink.detach();

        // Clean up finished sinks
        self.sfx_sinks.retain(|s| !s.empty());
        self.sfx_sinks.push(sink);

        Ok(())
    }

    /// Play background music (loops)
    pub fn play_music(&mut self, source: &AudioSource, looping: bool) -> Result<(), String> {
        // Stop existing music
        self.stop_music();

        let sink = Sink::try_new(&self.stream_handle)
            .map_err(|e| format!("Failed to create sink: {}", e))?;

        let decoder = source.decoder()?;
        sink.set_volume(self.master_volume * self.music_volume);
        
        if looping {
            sink.append(decoder.repeat_infinite());
        } else {
            sink.append(decoder);
        }

        self.music_sink = Some(sink);
        log::info!("Playing music (looping: {})", looping);

        Ok(())
    }

    /// Stop background music
    pub fn stop_music(&mut self) {
        if let Some(sink) = self.music_sink.take() {
            sink.stop();
        }
    }

    /// Pause background music
    pub fn pause_music(&self) {
        if let Some(sink) = &self.music_sink {
            sink.pause();
        }
    }

    /// Resume background music
    pub fn resume_music(&self) {
        if let Some(sink) = &self.music_sink {
            sink.play();
        }
    }

    /// Set master volume (0.0 to 1.0)
    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
        self.update_volumes();
    }

    /// Set music volume (0.0 to 1.0)
    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume.clamp(0.0, 1.0);
        self.update_volumes();
    }

    /// Set sound effects volume (0.0 to 1.0)
    pub fn set_sfx_volume(&mut self, volume: f32) {
        self.sfx_volume = volume.clamp(0.0, 1.0);
    }

    /// Update volume for all active sinks
    fn update_volumes(&self) {
        if let Some(sink) = &self.music_sink {
            sink.set_volume(self.master_volume * self.music_volume);
        }
    }

    /// Get master volume
    pub fn master_volume(&self) -> f32 {
        self.master_volume
    }

    /// Get music volume
    pub fn music_volume(&self) -> f32 {
        self.music_volume
    }

    /// Get SFX volume
    pub fn sfx_volume(&self) -> f32 {
        self.sfx_volume
    }
}

impl Default for AudioManager {
    fn default() -> Self {
        Self::new().expect("Failed to initialize audio manager")
    }
}
