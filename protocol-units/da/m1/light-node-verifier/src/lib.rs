pub mod celestia;

pub use m1_da_light_node_grpc::*;

/// A verified outcome. Indicates that input of A is verified as valid instance of B, or else invalid instance.
pub enum Verified<B> {
	Valid(B),
	Invalid,
}

#[tonic::async_trait]
pub trait Verifier<A, B>
where
	A: Send + Sync + 'static,
	B: Send + Sync + 'static,
{
	async fn verify(
		&self,
		verification_mode: VerificationMode,
		blob: A,
		height: u64,
	) -> Result<Verified<B>, anyhow::Error> {
		match verification_mode {
			VerificationMode::Cowboy => self.verify_cowboy(verification_mode, blob, height).await,
			VerificationMode::ValidatorIn => {
				self.verifiy_validator_in(verification_mode, blob, height).await
			}
			VerificationMode::MOfN => self.verify_m_of_n(verification_mode, blob, height).await,
		}
	}

	async fn verify_cowboy(
		&self,
		_verification_mode: VerificationMode,
		_blob: A,
		_height: u64,
	) -> Result<Verified<B>, anyhow::Error>;

	async fn verifiy_validator_in(
		&self,
		_verification_mode: VerificationMode,
		_blob: A,
		_height: u64,
	) -> Result<Verified<B>, anyhow::Error>;

	async fn verify_m_of_n(
		&self,
		_verification_mode: VerificationMode,
		_blob: A,
		_height: u64,
	) -> Result<Verified<B>, anyhow::Error>;
}
