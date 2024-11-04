# Keystroke Playback Programm

This is a tool to replay keystroke sequences.

## Prerequisites
To reproduce the experiments described in the paper"You Are as You Type: Examining the Impact of Timestamp Accuracy on the Robustness of Keystroke Biometrics", it is necessary to retrieve the fixed-text and free-text datasets as described in the Section "Dataset Sources" in this document. The fixed-text dataset needs to be placed into the root directory of this repository (see DSL-StrongPasswordData.csv as placeholder), and the free-text dataset needs to be placed to the directory /KEYSTROKE-SAMPLES-31-USERS/USERS , with samples for each user placed in a subfolder. See also the content of the JSON files in /KEYSTROKE-SAMPLES-31-USERS/ for details on required file paths.
## How to run

### With Rust installed

To run this app, you need to have [Rust](https://www.rust-lang.org/tools/install) installed.


Inside the repository directory, run

    cargo r -r -- -h

to get all available commands.

----

Get more information with 

    cargo r -r -- [<command>] -h


### Without Rust installed

#### Windows (stable-x86_64-pc-windows-msvc)
Inside the repository directory, run

    browser-timer-rs.exe -h

to get all available commands.

----

Get more information with 

    browser-timer-rs.exe [<command>] -h


## Dataset Sources

### Fixed-Text Dataset

The Dataset [DSL-StrongPasswordData.csv](./DSL-StrongPasswordData.csv) is part of the research paper [Comparing Anomaly-Detection Algorithms for Keystroke Dynamics](https://www.cs.cmu.edu/~maxion/pubs/KillourhyMaxion09.pdf).

Dataset: https://www.cs.cmu.edu/~keystroke/

### Free-Text Dataset

The Datasets [KEYSTROKE-SAMPLES-31-USERS](./KEYSTROKE-SAMPLES-31-USERS) is part of the research paper [Keystroke Analysis of Free Text](https://dl.acm.org/doi/pdf/10.1145/1085126.1085129). The Dataset can be obtained by contacting the autors of this paper.