// Copyright (c) The StackClass Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use hex;
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Generates an HMAC-SHA256 signature for the given payload using the provided
/// secret. Returns the signature as a hex-encoded string.
pub fn hmac_sha256_sign(payload: &str, secret: &str) -> Result<String, String> {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|e| format!("Failed to create HMAC: {}", e))?;
    mac.update(payload.as_bytes());
    let result = mac.finalize();

    Ok(hex::encode(result.into_bytes()))
}

/// Verifies an HMAC-SHA256 signature for the given payload using the provided
/// secret. Uses constant-time comparison to prevent timing attacks.
pub fn hmac_sha256_verify(payload: &str, secret: &str, sign: &str) -> Result<bool, String> {
    let expected = hmac_sha256_sign(payload, secret)?;

    // Constant-time comparison to prevent timing attacks
    Ok(subtle::ConstantTimeEq::ct_eq(sign.as_bytes(), expected.as_bytes()).into())
}
