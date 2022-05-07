# FST File Format

'FST' is the native format of 'GTKWave' but has seen almost
zero documentation outside of it. This doc intends to rectify 
that

## Features:

 - Storage of wave data (like VCD)
 - "Block" compression with support for random access
 - Efficient access of subsets of signals

## References:
 - FSTAPI from GTKWave's src/helpers/fst (copied and formatted here, because GNU indents give me headaches)
 - Implementation of an Efficient Method for Digital Waveform Compression (Anthony Bybell, AMD)