import ffmpeg
import os

# Arguments

overwrite = True
fps = 10
scale = 360


# Functions

def output_gif(stream_input, output_path, **kwargs):
    if overwrite:
        return ffmpeg.output(stream_input, output_path, format='gif', y=None,
                             filter_complex=f'[0]reverse[r];[0][r]concat=n=2:v=1:a=0,fps={fps},scale={scale}:-1')
    return ffmpeg.output(stream_input, output_path, format='gif',
                         filter_complex=f'[0]reverse[r];[0][r]concat=n=2:v=1:a=0,split[s0][s1];[s0]palettegen[p];['
                                        f's1][p]paletteuse,fps={fps},scale={scale}:-1')


# Init Variables

input_path = 'Input/'
input_files = [file for file in os.listdir(input_path) if not file.startswith('.')]

# Main Code

for file in input_files:
    filename, _ = os.path.splitext(file)
    stream = ffmpeg.input(input_path + file)
    stream = output_gif(stream, f'Output/output_{filename}.gif', overwrite=overwrite, fps=fps, scale=scale)
    ffmpeg.run(stream)