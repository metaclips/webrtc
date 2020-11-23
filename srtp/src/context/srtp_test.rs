//TODO: BenchmarkEncryptRTP
//TODO: BenchmarkEncryptRTPInPlace
//TODO: BenchmarkDecryptRTP
#[cfg(test)]
mod srtp_test {
    use crate::{context::Context, protection_profile::ProtectionProfile};
    use util::Error;

    use lazy_static::lazy_static;

    struct RTPTestCase {
        sequence_number: u16,
        encrypted: Vec<u8>,
    }

    lazy_static! {
        static ref RTP_TEST_CASE_DECRYPTED: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
        static ref RTP_TEST_CASES: Vec<RTPTestCase> = vec![
            RTPTestCase {
                sequence_number: 5000,
                encrypted: vec![
                    0x6d, 0xd3, 0x7e, 0xd5, 0x99, 0xb7, 0x2d, 0x28, 0xb1, 0xf3, 0xa1, 0xf0, 0xc, 0xfb,
                    0xfd, 0x8
                ],
            },
            RTPTestCase {
                sequence_number: 5001,
                encrypted: vec![
                    0xda, 0x47, 0xb, 0x2a, 0x74, 0x53, 0x65, 0xbd, 0x2f, 0xeb, 0xdc, 0x4b, 0x6d, 0x23,
                    0xf3, 0xde
                ],
            },
            RTPTestCase {
                sequence_number: 5002,
                encrypted: vec![
                    0x6e, 0xa7, 0x69, 0x8d, 0x24, 0x6d, 0xdc, 0xbf, 0xec, 0x2, 0x1c, 0xd1, 0x60, 0x76,
                    0xc1, 0x0e
                ],
            },
            RTPTestCase {
                sequence_number: 5003,
                encrypted: vec![
                    0x24, 0x7e, 0x96, 0xc8, 0x7d, 0x33, 0xa2, 0x92, 0x8d, 0x13, 0x8d, 0xe0, 0x76, 0x9f,
                    0x08, 0xdc
                ],
            },
            RTPTestCase {
                sequence_number: 5004,
                encrypted: vec![
                    0x75, 0x43, 0x28, 0xe4, 0x3a, 0x77, 0x59, 0x9b, 0x2e, 0xdf, 0x7b, 0x12, 0x68, 0x0b,
                    0x57, 0x49
                ],
            },
            RTPTestCase{
                sequence_number: 65535, // upper boundary
                encrypted: vec![
                    0xaf, 0xf7, 0xc2, 0x70, 0x37, 0x20, 0x83, 0x9c, 0x2c, 0x63, 0x85, 0x15, 0x0e, 0x44,
                    0xca, 0x36
                ],
            },
        ];
    }

    fn build_test_context() -> Result<Context, Error> {
        let master_key = vec![
            0x0d, 0xcd, 0x21, 0x3e, 0x4c, 0xbc, 0xf2, 0x8f, 0x01, 0x7f, 0x69, 0x94, 0x40, 0x1e,
            0x28, 0x89,
        ];

        let master_salt = vec![
            0x62, 0x77, 0x60, 0x38, 0xc0, 0x6d, 0xc9, 0x41, 0x9f, 0x6d, 0xd9, 0x43, 0x3e, 0x7c,
        ];

        Context::new(
            &master_key,
            &master_salt,
            ProtectionProfile::AES128CMHMACSHA1_80,
            None,
            None,
        )
    }

    #[test]
    fn test_rtp_invalid_auth() -> Result<(), Error> {
        let master_key = vec![
            0x0d, 0xcd, 0x21, 0x3e, 0x4c, 0xbc, 0xf2, 0x8f, 0x01, 0x7f, 0x69, 0x94, 0x40, 0x1e,
            0x28, 0x89,
        ];

        let invalid_salt = vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let mut encrypt_context = build_test_context()?;
        let mut invalid_context = Context::new(
            &master_key,
            &invalid_salt,
            ProtectionProfile::AES128CMHMACSHA1_80,
            None,
            None,
        )?;

        for test_case in &*RTP_TEST_CASES {
            let mut pkt = rtp::packet::Packet {
                header: rtp::header::Header {
                    sequence_number: test_case.sequence_number,
                    ..Default::default()
                },
                payload: RTP_TEST_CASE_DECRYPTED.clone(),
                ..Default::default()
            };

            let mut pkt_raw: Vec<u8> = pkt.marshal()?;

            let mut out = encrypt_context.encrypt_rtp(&mut pkt_raw)?;

            let result = invalid_context.decrypt_rtp(&mut out);

            assert!(
                result.is_err(),
                "Managed to decrypt with incorrect salt for packet with SeqNum: {}",
                test_case.sequence_number
            );
        }

        Ok(())
    }

    #[test]
    fn test_rtp_lifecyle() -> Result<(), Error> {
        let mut encrypt_context = build_test_context()?;
        let mut decrypt_context = build_test_context()?;
        let auth_tag_len = ProtectionProfile::AES128CMHMACSHA1_80.auth_tag_len();

        for test_case in RTP_TEST_CASES.iter() {
            let mut decrypted_pkt = rtp::packet::Packet {
                header: rtp::header::Header {
                    sequence_number: test_case.sequence_number,
                    ..Default::default()
                },

                payload: RTP_TEST_CASE_DECRYPTED.clone(),
                ..Default::default()
            };

            let mut decrypted_raw: Vec<u8> = decrypted_pkt.marshal()?;

            let mut encrypted_pkt = rtp::packet::Packet {
                header: rtp::header::Header {
                    sequence_number: test_case.sequence_number,
                    ..Default::default()
                },

                payload: test_case.encrypted.clone(),
                ..Default::default()
            };

            let mut encrypted_raw: Vec<u8> = encrypted_pkt.marshal()?;

            let actual_encrypted = encrypt_context.encrypt_rtp(&mut decrypted_raw)?;

            assert_eq!(
                actual_encrypted, encrypted_raw,
                "RTP packet with SeqNum invalid encryption: {}",
                test_case.sequence_number
            );

            let actual_decrypted = decrypt_context.decrypt_rtp(&mut encrypted_raw)?;

            assert_ne!(
                encrypted_raw[..encrypted_raw.len() - auth_tag_len].to_vec(),
                actual_decrypted,
                "DecryptRTP improperly encrypted in place"
            );

            assert_eq!(
                actual_decrypted, decrypted_raw,
                "RTP packet with SeqNum invalid decryption: {}",
                test_case.sequence_number,
            )
        }

        Ok(())
    }
}
