# Online Appendix and Artifacts for  *You Are as You Type: Examining the Impact of Timestamp Accuracy on the Robustness of Keystroke Biometrics*

## Online Appendix
The file *Online_Appendix.pdf* contains Tables V and VI.

## Scripts and Applications used in  *You Are as You Type: Examining the Impact of Timestamp Accuracy on the Robustness of Keystroke Biometrics*

The folder [Artifacts](./Artifacts/) contains all scripts and applications that were used for the paper *You Are as You Type: Examining the Impact of Timestamp Accuracy on the Robustness of Keystroke Biometrics*.

The following is a short description and note of where each application was used/referenced in the paper.


### [keystroke-playback-programm](./Artifacts/keystroke-playback-programm/)
Programm for simulating keystrokes written in Rust. This application is described in Section 3 of the paper.

----

### [browser-timer](./Artifacts/keystroke-recording-programm/)
Applications for receiving/recording keystrokes in web browsers. 


#### [src](./Artifacts/keystroke-recording-programm/src/)
- Webapp (static HTML resources) for recording keystrokes in web browsers. Used to record data for the experiments presented in Section 4.1 and 4.2 of the paper.

#### [browser-timer-ws](./Artifacts/keystroke-recording-programm/browser-timer-ws/)
- Websocket/HTTP server for receiving keystrokes. Used to record data for the experiments presented in Section 4.3 of the paper.

----

### [algorithms](./Artifacts/algorithms/)
This section contains scripts for analyzing keystroke dynamics data.

#### [experiment-setup-precision](./Artifacts/algorithms/experiment-setup-precision/analysis.ipynb)
- Script for analysing keystroke dynamics data retrieved from the pre-experiement described in Section 3.2 of the Paper.


#### [fixed-text](./Artifacts/algorithms/fixed-text/kd-analysis-fixed-text.ipynb)
- Implementation of the algorithm presented in *Kevin S. Killourhy and Roy A. Maxion. 2009. Comparing anomaly-detection algorithms for keystroke dynamics. In 2009 IEEE/IFIP International Conference on Dependable Systems & Networks. IEEE, Lisbon, Portugal, 125–134*
- Used for fixed-text experiments


#### [free-text](./Artifacts/algorithms/free-text/kd-analysis-free-text.ipynb)
- Our implementation of the algorithm presented in *Daniele Gunetti and Claudia Picardi. 2005. Keystroke analysis of free text. ACM, Transactions on Information and System Security 8, 3 (Aug. 2005), 312–347*
- Used for free-text experiments
