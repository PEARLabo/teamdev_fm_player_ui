// // サンプルデータ
// #[tauri::command]
// fn send_file_test(window: Window, contents: Vec<u8>, _port_name: String) -> Result<(), String> {
//     // サンプルデータの送信
// let sample_data = vec![
//     "Starting playback info".to_string(),
//     "[1,1,1,1,65]".to_string(),
//     "chanel: 1( 1), key: 1(     1), velocity: 65(         65)".to_string(),
//     "[1,64,66,15,1]".to_string(),
//     "DecayRate: 2(     2), change param: 15(         15)".to_string(),
//     "[113,53,115,113,117]".to_string(),
//     "[97,114,101,113,85]".to_string(),
//     "FlagA is invalid: 7".to_string(),
//     "[115,97,119,32,32]".to_string(),
//     "FlagA is invalid: 6".to_string(),
//     "[32,1,65,1,141]".to_string(),
//     "chanel: 1( 1), key: 1(     1), velocity: 141(        141)".to_string(),
//     "[209,4,1,49,32]".to_string(),
//     "chanel: 4( 4), key: 49(    49), velocity: 32(         32)".to_string(),
//     "[80,48,49,48,100]".to_string(),
//     "[64,49,80,100,48]".to_string(),
//     "[49,4,0,240,49]".to_string(),
//     "chanel: 4( 4), key: 240(   240), velocity: 49(         49)".to_string(),
//     "[4,7,60,49,4]".to_string(),
//     "chanel: 7( 7), key: 49(    49), velocity: 4(          4)".to_string(),
//     "[1,50,49,4,17]".to_string(),
//     "[50,49,4,33,113]".to_string(),
//     "[49,4,49,66,49]".to_string(),
//     "chanel: 4( 4), key: 66(    66), velocity: 49(         49)".to_string(),
//     "[4,2,30,49,4]".to_string(),
//     "chanel: 2( 2), key: 49(    49), velocity: 4(          4)".to_string(),
//     "[18,16,49,4,34]".to_string(),
//     "tempo: 3212322(U24(3212322))[μsec/四分音符], BPM: 0".to_string(),
//     "[12,49,4,50,16]".to_string(),
//     "[49,4,4,7,49]".to_string(),
//     "chanel: 4( 4), key: 7(     7), velocity: 49(         49)".to_string(),
//     "[4,20,31,49,4]".to_string(),
//     "tempo: 2044164(U24(2044164))[μsec/四分音符], BPM: 0".to_string(),
//     "[36,7,49,4,52]".to_string(),
//     "chanel: 7( 7), key: 4(     4), velocity: 52(         52)".to_string(),
//     "[31,49,4,6,19]".to_string(),
//     "[49,4,22,6,49]".to_string(),
//     "chanel: 4( 4), key: 6(     6), velocity: 49(         49)".to_string(),
//     "[4,38,19,49,4]".to_string(),
//     "End".to_string(),
//     "[54,6,49,4,3]".to_string(),
//     "chanel: 6( 6), key: 4(     4), velocity: 3(          3)".to_string(),
//     "[31,49,4,19,24]".to_string(),
//     "[49,4,35,31,49]".to_string(),
//     "chanel: 4( 4), key: 31(    31), velocity: 49(         49)".to_string(),
//     "[4,51,30,49,4]".to_string(),
//     "[5,0,49,4,21]".to_string(),
//     "chanel: 0( 0), key: 4(     4), velocity: 21(         21)".to_string(),
//     "[0,49,4,37,0]".to_string(),
//     "[49,4,53,0,49]".to_string(),
//     "chanel: 4( 4), key: 0(     0), velocity: 49(         49)".to_string(),
//     "[20,0,241,49,20]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 20(         20)".to_string(),
//     "[7,60,49,20,1]".to_string(),
//     "[51,49,20,17,52]".to_string(),
//     "[49,20,33,50,49]".to_string(),
//     "tempo: 2175537(U24(2175537))[μsec/四分音符], BPM: 0".to_string(),
//     "[20,49,49,49,20]".to_string(),
//     "[2,0,49,20,18]".to_string(),
//     "chanel: 0( 0), key: 20(    20), velocity: 18(         18)".to_string(),
//     "[0,49,20,34,0]".to_string(),
//     "[49,20,50,0,49]".to_string(),
//     "tempo: 3276849(U24(3276849))[μsec/四分音符], BPM: 0".to_string(),
//     "[20,4,1,49,20]".to_string(),
//     "chanel: 4( 4), key: 49(    49), velocity: 20(         20)".to_string(),
//     "[20,26,49,20,36]".to_string(),
//     "tempo: 3216420(U24(3216420))[μsec/四分音符], BPM: 0".to_string(),
//     "[25,49,20,52,20]".to_string(),
//     "[49,20,6,7,49]".to_string(),
//     "tempo: 395057(U24(395057))[μsec/四分音符], BPM: 2".to_string(),
//     "[20,22,27,49,20]".to_string(),
//     "tempo: 1782036(U24(1782036))[μsec/四分音符], BPM: 0".to_string(),
//     "[38,200,49,20,54]".to_string(),
//     "FlagA is invalid: 12".to_string(),
//     "[252,49,20,3,30]".to_string(),
//     "[49,20,19,30,49]".to_string(),
//     "tempo: 1252913(U24(1252913))[μsec/四分音符], BPM: 0".to_string(),
//     "[20,35,30,49,20]".to_string(),
//     "End".to_string(),
//     "[51,30,49,20,5]".to_string(),
//     "tempo: 3216389(U24(3216389))[μsec/四分音符], BPM: 0".to_string(),
//     "[1,49,20,21,16]".to_string(),
//     "[49,20,37,10,49]".to_string(),
//     "tempo: 2427441(U24(2427441))[μsec/四分音符], BPM: 0".to_string(),
//     "[20,53,10,1,1]".to_string(),
//     "[113,37,56,48,56]".to_string(),
//     "End".to_string(),
//     "[107,105,107,49,32]".to_string(),
//     "FlagA is invalid: 6".to_string(),
//     "[0,48,49,48,0]".to_string(),
//     "[64,49,80,0,48]".to_string(),
//     "[49,32,80,48,49]".to_string(),
//     "End".to_string(),
//     "[48,100,64,49,80]".to_string(),
//     "FlagA is invalid: 6".to_string(),
//     "[100,48,49,32,0]".to_string(),
//     "[48,49,48,0,64]".to_string(),
//     "[49,80,0,48,49]".to_string(),
//     "FlagA is 5: Skip to next track.".to_string(),
//     "[32,80,48,49,48]".to_string(),
//     "FlagA is 5: Skip to next track.".to_string(),
//     "[100,62,49,80,100]".to_string(),
//     "[46,49,32,0,48]".to_string(),
//     "[49,48,0,62,49]".to_string(),
//     "[80,0,46,49,32]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 32(         32)".to_string(),
//     "[80,48,49,48,100]".to_string(),
//     "[50,113,69,115,113]".to_string(),
//     "FlagA is invalid: 7".to_string(),
//     "[117,97,114,101,49]".to_string(),
//     "FlagA is invalid: 6".to_string(),
//     "[64,100,62,49,32]".to_string(),
//     "FlagA is invalid: 6".to_string(),
//     "[0,48,49,48,0]".to_string(),
//     "[50,49,64,0,62]".to_string(),
//     "[49,48,100,50,49]".to_string(),
//     "[64,100,60,49,48]".to_string(),
//     "FlagA is invalid: 6".to_string(),
//     "[0,50,49,64,0]".to_string(),
//     "[60,49,16,100,50]".to_string(),
//     "[49,64,100,62,49]".to_string(),
//     "SustainLevel/ReleaseRate: 4(     4), change param: 62(         62)".to_string(),
//     "[64,0,62,49,16]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 16(         16)".to_string(),
//     "[0,50,49,16,100]".to_string(),
//     "[50,49,48,100,52]".to_string(),
//     "[49,64,100,69,49]".to_string(),
//     "SustainLevel/ReleaseRate: 4(     4), change param: 69(         69)".to_string(),
//     "[48,0,52,49,64]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 64(         64)".to_string(),
//     "[0,69,49,48,100]".to_string(),
//     "KeyScale/AttackRate: 1(     1), change param: 48(         48)".to_string(),
//     "[52,49,48,0,52]".to_string(),
//     "[49,16,0,50,49]".to_string(),
//     "tempo: 12849(U24(12849))[μsec/四分音符], BPM: 77".to_string(),
//     "[32,80,48,49,64]".to_string(),
//     "FlagA is 5: Skip to next track.".to_string(),
//     "[100,62,49,32,0]".to_string(),
//     "[48,49,64,0,62]".to_string(),
//     "[49,16,100,50,49]".to_string(),
//     "tempo: 6566449(U24(6566449))[μsec/四分音符], BPM: 0".to_string(),
//     "[48,100,53,49,48]".to_string(),
//     "FlagA is invalid: 6".to_string(),
//     "[0,53,49,48,100]".to_string(),
//     "[53,49,64,100,60]".to_string(),
//     "[49,48,0,53,49]".to_string(),
//     "[64,0,60,49,16]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 16(         16)".to_string(),
//     "[0,50,49,32,80]".to_string(),
//     "[48,49,32,0,48]".to_string(),
//     "[49,64,100,62,49]".to_string(),
//     "SustainLevel/ReleaseRate: 4(     4), change param: 62(         62)".to_string(),
//     "[64,0,62,49,16]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 16(         16)".to_string(),
//     "[100,50,49,64,100]".to_string(),
//     "[64,49,64,0,64]".to_string(),
//     "[49,64,100,65,49]".to_string(),
//     "SustainLevel/ReleaseRate: 4(     4), change param: 65(         65)".to_string(),
//     "[64,0,65,49,16]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 16(         16)".to_string(),
//     "[0,50,49,32,80]".to_string(),
//     "[48,49,64,100,64]".to_string(),
//     "[49,32,0,48,49]".to_string(),
//     "[64,0,64,49,32]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 32(         32)".to_string(),
//     "[80,48,49,64,100]".to_string(),
//     "[62,49,32,0,48]".to_string(),
//     "[49,64,0,62,49]".to_string(),
//     "Slot: 0(     0), change param: 62(         62)".to_string(),
//     "[16,100,50,49,64]".to_string(),
//     "FlagA is invalid: 6".to_string(),
//     "[100,60,49,64,0]".to_string(),
//     "[60,49,64,100,57]".to_string(),
//     "[49,64,0,57,49]".to_string(),
//     "Slot: 0(     0), change param: 57(         57)".to_string(),
//     "[16,0,50,49,32]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 32(         32)".to_string(),
//     "[80,48,49,48,100]".to_string(),
//     "[50,49,64,100,62]".to_string(),
//     "[49,32,0,48,49]".to_string(),
//     "[48,100,50,49,64]".to_string(),
//     "FlagA is invalid: 6".to_string(),
//     "[100,62,49,32,0]".to_string(),
//     "[48,49,48,0,50]".to_string(),
//     "[49,64,0,62,49]".to_string(),
//     "Slot: 0(     0), change param: 62(         62)".to_string(),
//     "[0,0,65,49,0]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 0(          0)".to_string(),
//     "[100,65,49,48,100]".to_string(),
//     "KeyScale/AttackRate: 1(     1), change param: 48(         48)".to_string(),
//     "[50,49,64,100,60]".to_string(),
//     "[49,48,0,50,49]".to_string(),
//     "[64,0,60,49,0]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 0(          0)".to_string(),
//     "[0,65,49,0,100]".to_string(),
//     "KeyScale/AttackRate: 1(     1), change param: 0(          0)".to_string(),
//     "[65,49,16,100,50]".to_string(),
//     "[49,64,100,62,49]".to_string(),
//     "SustainLevel/ReleaseRate: 4(     4), change param: 62(         62)".to_string(),
//     "[64,0,62,49,0]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 0(          0)".to_string(),
//     "[0,65,49,0,100]".to_string(),
//     "KeyScale/AttackRate: 1(     1), change param: 0(          0)".to_string(),
//     "[65,49,16,0,50]".to_string(),
//     "[49,16,100,50,49]".to_string(),
//     "tempo: 6566449(U24(6566449))[μsec/四分音符], BPM: 0".to_string(),
//     "[48,100,52,49,64]".to_string(),
//     "FlagA is invalid: 6".to_string(),
//     "[100,69,49,48,0]".to_string(),
//     "KeyScale/AttackRate: 1(     1), change param: 48(         48)".to_string(),
//     "[52,49,64,0,69]".to_string(),
//     "[49,0,0,65,49]".to_string(),
//     "chanel: 0( 0), key: 65(    65), velocity: 49(         49)".to_string(),
//     "[0,100,65,49,48]".to_string(),
//     "FlagA is invalid: 6".to_string(),
//     "[100,52,49,48,0]".to_string(),
//     "[52,49,0,0,65]".to_string(),
//     "[49,0,100,64,49]".to_string(),
//     "chanel: 0( 0), key: 64(    64), velocity: 49(         49)".to_string(),
//     "[16,0,50,49,32]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 32(         32)".to_string(),
//     "[80,48,49,64,100]".to_string(),
//     "[62,49,32,0,48]".to_string(),
//     "[49,64,0,62,49]".to_string(),
//     "Slot: 0(     0), change param: 62(         62)".to_string(),
//     "[0,0,64,49,0]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 0(          0)".to_string(),
//     "[100,62,49,16,100]".to_string(),
//     "[50,49,48,100,53]".to_string(),
//     "[49,48,0,53,49]".to_string(),
//     "[0,0,62,49,0]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 0(          0)".to_string(),
//     "[100,60,49,48,100]".to_string(),
//     "[53,49,64,100,60]".to_string(),
//     "[49,48,0,53,49]".to_string(),
//     "[64,0,60,49,0]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 0(          0)".to_string(),
//     "[0,60,49,0,100]".to_string(),
//     "[62,49,16,0,50]".to_string(),
//     "[49,32,80,48,49]".to_string(),
//     "End".to_string(),
//     "[32,0,48,49,64]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 64(         64)".to_string(),
//     "[100,62,49,64,0]".to_string(),
//     "[62,49,0,0,62]".to_string(),
//     "[49,0,100,60,49]".to_string(),
//     "chanel: 0( 0), key: 60(    60), velocity: 49(         49)".to_string(),
//     "[16,100,50,49,64]".to_string(),
//     "FlagA is invalid: 6".to_string(),
//     "[100,64,49,64,0]".to_string(),
//     "KeyScale/AttackRate: 1(     1), change param: 64(         64)".to_string(),
//     "[64,49,64,100,65]".to_string(),
//     "[49,64,0,65,49]".to_string(),
//     "Slot: 0(     0), change param: 65(         65)".to_string(),
//     "[0,0,60,49,0]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 0(          0)".to_string(),
//     "[100,57,49,16,0]".to_string(),
//     "[50,49,32,80,48]".to_string(),
//     "[49,32,0,48,49]".to_string(),
//     "[32,0,48,49,64]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 64(         64)".to_string(),
//     "[0,64,49,32,80]".to_string(),
//     "KeyScale/AttackRate: 1(     1), change param: 32(         32)".to_string(),
//     "[48,49,64,100,62]".to_string(),
//     "[49,32,0,48,49]".to_string(),
//     "[64,0,62,49,0]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 0(          0)".to_string(),
//     "[0,57,49,16,100]".to_string(),
//     "[50,49,64,100,60]".to_string(),
//     "[49,64,0,60,49]".to_string(),
//     "Slot: 0(     0), change param: 60(         60)".to_string(),
//     "[0,100,57,49,64]".to_string(),
//     "FlagA is invalid: 6".to_string(),
//     "[100,57,49,64,0]".to_string(),
//     "[57,49,0,0,57]".to_string(),
//     "[49,0,100,65,49]".to_string(),
//     "chanel: 0( 0), key: 65(    65), velocity: 49(         49)".to_string(),
//     "[16,0,50,49,32]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 32(         32)".to_string(),
//     "[80,48,49,48,100]".to_string(),
//     "[50,49,64,100,62]".to_string(),
//     "[49,32,0,48,49]".to_string(),
//     "[48,0,50,49,64]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 64(         64)".to_string(),
//     "[0,62,49,0,0]".to_string(),
//     "[65,49,0,100,65]".to_string(),
//     "[49,48,100,50,49]".to_string(),
//     "[64,100,60,49,48]".to_string(),
//     "FlagA is invalid: 6".to_string(),
//     "[0,50,49,64,0]".to_string(),
//     "[60,49,0,0,65]".to_string(),
//     "[49,0,100,65,49]".to_string(),
//     "chanel: 0( 0), key: 65(    65), velocity: 49(         49)".to_string(),
//     "[16,100,50,49,64]".to_string(),
//     "FlagA is invalid: 6".to_string(),
//     "[100,62,49,64,0]".to_string(),
//     "[62,49,0,0,65]".to_string(),
//     "[49,0,100,65,49]".to_string(),
//     "chanel: 0( 0), key: 65(    65), velocity: 49(         49)".to_string(),
//     "[16,0,50,49,16]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 16(         16)".to_string(),
//     "[100,50,49,48,100]".to_string(),
//     "[52,49,64,100,69]".to_string(),
//     "[49,48,0,52,49]".to_string(),
//     "[64,0,69,49,0]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 0(          0)".to_string(),
//     "[0,65,49,0,100]".to_string(),
//     "KeyScale/AttackRate: 1(     1), change param: 0(          0)".to_string(),
//     "[65,49,48,100,52]".to_string(),
//     "[49,48,0,52,49]".to_string(),
//     "[0,0,65,49,0]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 0(          0)".to_string(),
//     "[100,64,49,16,0]".to_string(),
//     "KeyScale/AttackRate: 1(     1), change param: 16(         16)".to_string(),
//     "[50,49,32,80,48]".to_string(),
//     "[49,64,100,62,49]".to_string(),
//     "SustainLevel/ReleaseRate: 4(     4), change param: 62(         62)".to_string(),
//     "[32,0,48,49,64]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 64(         64)".to_string(),
//     "[0,62,49,0,0]".to_string(),
//     "[64,49,0,100,62]".to_string(),
//     "[49,16,100,50,49]".to_string(),
//     "tempo: 6566449(U24(6566449))[μsec/四分音符], BPM: 0".to_string(),
//     "[48,100,48,49,48]".to_string(),
//     "FlagA is invalid: 6".to_string(),
//     "[0,48,49,0,0]".to_string(),
//     "[62,49,0,100,60]".to_string(),
//     "[49,48,100,48,49]".to_string(),
//     "[64,100,60,49,48]".to_string(),
//     "FlagA is invalid: 6".to_string(),
//     "[0,48,49,64,0]".to_string(),
//     "[60,49,0,0,60]".to_string(),
//     "[49,0,100,62,49]".to_string(),
//     "chanel: 0( 0), key: 62(    62), velocity: 49(         49)".to_string(),
//     "[16,0,50,49,32]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 32(         32)".to_string(),
//     "[80,48,49,32,0]".to_string(),
//     "[48,49,64,100,62]".to_string(),
//     "[49,64,0,62,49]".to_string(),
//     "Slot: 0(     0), change param: 62(         62)".to_string(),
//     "[0,0,62,49,0]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 0(          0)".to_string(),
//     "[100,60,49,16,100]".to_string(),
//     "[50,49,32,80,60]".to_string(),
//     "[49,64,100,64,49]".to_string(),
//     "SustainLevel/ReleaseRate: 4(     4), change param: 64(         64)".to_string(),
//     "[32,0,60,49,64]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 64(         64)".to_string(),
//     "[0,64,49,32,80]".to_string(),
//     "KeyScale/AttackRate: 1(     1), change param: 32(         32)".to_string(),
//     "[60,49,64,100,65]".to_string(),
//     "[49,32,0,60,49]".to_string(),
//     "[64,0,65,49,0]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 0(          0)".to_string(),
//     "[0,60,49,0,100]".to_string(),
//     "[62,49,16,0,50]".to_string(),
//     "[49,32,80,48,49]".to_string(),
//     "[64,100,64,49,32]".to_string(),
//     "FlagA is invalid: 6".to_string(),
//     "[0,48,49,32,80]".to_string(),
//     "[48,49,32,0,48]".to_string(),
//     "[49,64,0,64,49]".to_string(),
//     "Slot: 0(     0), change param: 64(         64)".to_string(),
//     "[0,0,62,49,16]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 16(         16)".to_string(),
//     "[100,50,49,0,100]".to_string(),
//     "[57,49,32,80,48]".to_string(),
//     "[49,32,0,48,49]".to_string(),
//     "[0,0,57,49,0]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 0(          0)".to_string(),
//     "[100,65,49,16,0]".to_string(),
//     "KeyScale/AttackRate: 1(     1), change param: 16(         16)".to_string(),
//     "[50,49,32,80,48]".to_string(),
//     "[49,48,100,50,49]".to_string(),
//     "[64,100,62,49,32]".to_string(),
//     "FlagA is invalid: 6".to_string(),
//     "[0,48,49,48,0]".to_string(),
//     "[50,49,64,0,62]".to_string(),
//     "[49,0,0,65,49]".to_string(),
//     "chanel: 0( 0), key: 65(    65), velocity: 49(         49)".to_string(),
//     "[0,100,65,49,48]".to_string(),
//     "FlagA is invalid: 6".to_string(),
//     "[100,50,49,64,100]".to_string(),
//     "[60,49,48,0,50]".to_string(),
//     "[49,64,0,60,49]".to_string(),
//     "Slot: 0(     0), change param: 60(         60)".to_string(),
//     "[0,0,65,49,0]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 0(          0)".to_string(),
//     "[100,65,49,16,100]".to_string(),
//     "KeyScale/AttackRate: 1(     1), change param: 16(         16)".to_string(),
//     "[50,49,64,100,62]".to_string(),
//     "[49,64,0,62,49]".to_string(),
//     "Slot: 0(     0), change param: 62(         62)".to_string(),
//     "[0,0,65,49,0]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 0(          0)".to_string(),
//     "[100,65,49,16,0]".to_string(),
//     "KeyScale/AttackRate: 1(     1), change param: 16(         16)".to_string(),
//     "[50,49,16,100,50]".to_string(),
//     "[49,48,100,52,49]".to_string(),
//     "[64,100,69,49,48]".to_string(),
//     "FlagA is invalid: 6".to_string(),
//     "[0,52,49,64,0]".to_string(),
//     "[69,49,0,0,65]".to_string(),
//     "[49,0,100,65,49]".to_string(),
//     "chanel: 0( 0), key: 65(    65), velocity: 49(         49)".to_string(),
//     "[48,100,52,49,48]".to_string(),
//     "FlagA is invalid: 6".to_string(),
//     "[0,52,49,0,0]".to_string(),
//     "[65,49,0,100,64]".to_string(),
//     "[49,16,0,50,49]".to_string(),
//     "tempo: 12849(U24(12849))[μsec/四分音符], BPM: 77".to_string(),
//     "[32,80,48,49,64]".to_string(),
//     "FlagA is 5: Skip to next track.".to_string(),
//     "[100,62,49,32,0]".to_string(),
//     "[48,49,64,0,62]".to_string(),
//     "[49,0,0,64,49]".to_string(),
//     "chanel: 0( 0), key: 64(    64), velocity: 49(         49)".to_string(),
//     "[0,100,62,49,16]".to_string(),
//     "FlagA is invalid: 6".to_string(),
//     "[100,50,49,48,100]".to_string(),
//     "[53,49,48,0,53]".to_string(),
//     "[49,0,0,62,49]".to_string(),
//     "chanel: 0( 0), key: 62(    62), velocity: 49(         49)".to_string(),
//     "[0,100,60,49,48]".to_string(),
//     "FlagA is invalid: 6".to_string(),
//     "[100,53,49,64,100]".to_string(),
//     "[60,49,48,0,53]".to_string(),
//     "[49,64,0,60,49]".to_string(),
//     "Slot: 0(     0), change param: 60(         60)".to_string(),
//     "[0,0,60,49,0]".to_string(),
//     "chanel: 0( 0), key: 49(    49), velocity: 0(          0)".to_string(),
//     "[100,62,49,16,0]".to_string(),
//     "[50,49,32,80,48]".to_string(),
//     "[49,32,0,48,49]".to_string(),
//     "End".to_string()
// ];

//     for (i, data) in sample_data.iter().enumerate() {
//         let window = window.clone();
//         let data = data.clone();
//         std::thread::spawn(move || {
//             std::thread::sleep(std::time::Duration::from_millis((i as u64 * 1000) / 2));
//             window.emit("playback_info", data).unwrap();
//         });
//     }

//     Ok(())
// }
