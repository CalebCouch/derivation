use bitcoin::bip32::{Xpriv, Xpub, ChildNumber};
use bitcoin::secp256k1::rand;
use bitcoin::secp256k1::scalar::Scalar;

fn main() {
    let ctx = bitcoin::key::Secp256k1::new();
    let master = bitcoin::secp256k1::SecretKey::new(&mut rand::thread_rng());
    let public = master.public_key(&ctx);

    let master_x = Xpriv::new_master(bitcoin::Network::Bitcoin, &master.secret_bytes()).unwrap();
    let public_x = Xpub::from_priv(&ctx, &master_x);

    let public_0 = public_x.derive_pub(&ctx, &[ChildNumber::from_normal_idx(1).unwrap()]).unwrap();
    let public_c = public.combine(&public_0.public_key).unwrap();

    let secret_0 = master_x.derive_priv(&ctx, &[ChildNumber::from_normal_idx(1).unwrap()]).unwrap();
    let secret_c = master.add_tweak(&Scalar::from_be_bytes(*secret_0.private_key.as_ref()).unwrap()).unwrap();

    assert!(public_c == secret_c.public_key(&ctx));
    //Given secret_c and public_x, Can master, master_x or secret_0 be discovered???
}
