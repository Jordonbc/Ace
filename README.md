# Advanced Chunk Encoder (ACE)

ACE, short for Advanced Chunk Encoder, is a Rust Tauri application built to streamline the process of splitting videos into chunks and reencoding them. It aims to provide a more robust and efficient solution compared to NEAV1E, offering the flexibility of all FFmpeg options, with the added convenience of saving/loading encoder profiles and a pause/resume feature that saves its progress on disk—even surviving PC restarts.

![ACE Screenshot](path/to/screenshot.jpg) *Optional: Place a screenshot of your application here*

## Features

- **Full FFmpeg Options Integration**: Tailor your encoding process with the entire range of FFmpeg functionalities.
  
- **Encoder Profiles**: Save and load your favorite or frequently used encoder configurations for a seamless reencoding experience.

- **Persistent Pause and Resume**: Pause your encoding and pick up right where you left off—even after a system restart.

## Technologies Used

- [**Rust**](https://www.rust-lang.org): For core logic and performance optimization.
  
- [**Tauri**](https://tauri.app): To provide a lightweight and robust desktop application experience.

## Installation

### Using Compiled Binaries

Pre-compiled binaries are automatically generated whenever changes are pushed or merged into the `stable` branch. You can download the latest binary for your platform from the [releases page](https://github.com/Jordonbc/Ace/releases).

### Building from Source

If you prefer to build ACE from the source code:

1. Ensure you have Rust and Cargo installed on your system. If not, [install Rust](https://rustup.rs/).

2. Clone the repository:

```bash
git clone https://github.com/Jordonbc/Ace.git
```
3. Navigate to the project directory:
```bash
cd Ace
```
4. Build the project using Cargo:
```bash
cargo build --release
```

This will generate a binary in the target/release directory.

## Usage

Using ACE is straightforward. Once you've launched the application, follow the steps below to reencode your selected video:

1. **Select Your Video**: Browse and choose the video you want to reencode.

2. **Choose Encoding Settings**: Configure the desired encoding settings using the provided options, which integrate with FFmpeg.

3. **Start the Reencoding Process**: Once your video and settings are selected, press the "Start Reencoding" button. ACE will handle the following steps automatically:
    - **Process Audio**: The audio from the selected video is extracted and processed first.
    - **Split Video**: The video is then split into manageable chunks.
    - **Parallel Chunked Encoding**: Each chunk is encoded simultaneously in parallel for faster processing.
    - **Merge Chunks**: After encoding, the chunks are merged back into a single video stream.
    - **Merge Audio**: The processed audio is then merged back with the video to complete the reencoding.

4. **Subtitles and Additional Features**: If your video includes subtitles, the application will handle them similarly. They will be extracted, processed, and merged back into the final reencoded video. Make sure to select the appropriate options if your video includes subtitles or any additional tracks.

5. Once the process is complete, save your reencoded video to your preferred location.

Enjoy your newly reencoded video with the chosen settings!

## Contribution
Contributions are always welcome! Please ensure that you adhere to the following guidelines:
- Fork the repository and create your branch from `dev`.
- Submit a pull request and provide a clear description of your changes.

## FAQ
Q1: What video formats does ACE support?
A1: ACE, thanks to its FFmpeg integration, supports a wide range of video formats including MP4, AVI, MKV, and more. If FFmpeg can handle it, ACE can too.

Q2: I encountered an error during the reencoding process. What should I do?
A2: Firstly, ensure that your video file isn't corrupted. If the problem persists, please report the issue on our GitHub issues page, providing as much detail as possible.

Q3: Can ACE handle 4K videos?
A3: Absolutely. ACE is designed to manage videos of various resolutions, including 4K. However, the processing time may vary based on the video's length and your computer's performance.

## Acknowledgments
- FFmpeg: The backbone of our encoding process, allowing for versatile video processing. Check them out.

- Tauri: For making it possible to deliver ACE as a lightweight and efficient desktop application.

## License
This project is licensed under the MIT License. Refer to the [LICENSE](https://github.com/Jordonbc/Ace/blob/stable/LICENSE) file for detailed information.