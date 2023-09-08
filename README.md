# Video Player Console R

A video player for your console written in rust

## Command Line Arguments

- `--fps` \<float bigger than 0>
- `--mode` \<mode id>
- `--audio` \<true | false>
- `--option` \<any natural number or 0>

## Modes

0. True Color Mode
1. Limited Color Mode
   - mode option = color divider
2. ASCII Mode
3. Dithered ASCII Mode
   - mode option = dither
4. Block Mode
5. Dithered Block Mode
   - mode option = dither

## Performance

Performance is bad if you want a high resolution

### Least Dropped frames on a high res

- Limited Color
- Mode Option: 255

### Most usable result

- Limited Color Mode
- Mode Option: 10

### Best Graphics

- True Color Mode
