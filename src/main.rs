use bitcoin::bip32::{Xpriv, Xpub, ChildNumber};
use bitcoin::secp256k1::rand;
use bitcoin::secp256k1::scalar::Scalar;

fn main() {
    let ctx = bitcoin::key::Secp256k1::new();
    let master = bitcoin::secp256k1::SecretKey::new(&mut rand::thread_rng());
    let public = master.public_key(&ctx);

    let master_x = Xpriv::new_master(bitcoin::Network::Bitcoin, &master.secret_bytes()).unwrap();
    let public_x = Xpub::from_priv(&ctx, &master_x);

    let protocol = public_x.derive_pub(&ctx, &[ChildNumber::from_normal_idx(1).unwrap()]).unwrap();

    let actor_pub = public.combine(&protocol.public_key).unwrap();

    let actor_priv = master.add_tweak(&Scalar::from_be_bytes(
        *master_x.derive_priv(&ctx, &[ChildNumber::from_normal_idx(1).unwrap()]).unwrap().private_key.as_ref()
    ).unwrap()).unwrap();

    assert!(actor_pub == actor_priv.public_key(&ctx));
}
