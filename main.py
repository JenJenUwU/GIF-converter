import ffmpeg
import os

# Arguments

input_path = 'Input/'
input_files = [file for file in os.listdir(input_path) if not file.startswith('.')]


# Functions

def output_gif(input_file, output_path, size=640, fps=10):
    filename = os.path.splitext(os.path.basename(input_file))[0]
    palette_file = os.path.join(output_path, f'{filename}_palette.png')
    output_file = os.path.join(output_path, f'{filename}.gif')

    stream = ffmpeg.input(input_file)
    stream = ffmpeg.filter(stream, 'scale', size, -1, flags='lanczos')
    ffmpeg.input(input_file).output(palette_file, vf='palettegen').run(overwrite_output=True)
    (
        ffmpeg.filter(
            [
                stream,
                ffmpeg.input(palette_file)
            ],
            filter_name='paletteuse',
        )
        .filter('fps', fps=fps)
        .output(output_file, format='gif', loop=0)
        .run(overwrite_output=True)
    )


# Main Code

for file in input_files:
    output_gif(input_path + file, "Output", size=640, fps=25)
