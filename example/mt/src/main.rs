use merkle_cbt::{merkle_tree::Merge, CBMT as ExCBMT};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

pub struct DefaultHasherU64;

impl Merge for DefaultHasherU64 {
    type Item = u64;
    fn merge(left: &Self::Item, right: &Self::Item) -> Self::Item {
        let mut hasher = DefaultHasher::new();
        hasher.write_u64(*left);
        hasher.write_u64(*right);
        hasher.finish()
    }
}

type CBMT = ExCBMT<u64, DefaultHasherU64>;

fn main() {


    let leaves_1 = vec![
        "\\x404280d1fd9bbe5569bd3c2a17006041a8529b76743124df9b2017e96339c831",
        "0x404280d1fd9bbe5569bd3c2a17006041a8529b76743124df9b2017e96339c831",
        "11643453954163878810",
        "11177097603989645559",
        "20191116",
        "10289152030157698709",
    ];

    let leaves: Vec<_> =  leaves_1.iter().map(|x| { 
        let mut hasher = DefaultHasher::new();
        hasher.write(x.as_bytes());
        hasher.finish()
    }).collect();


    println!("leaves convert from string: {:#?} => \n u64 : {:#?} ", leaves_1, leaves );

    let root = CBMT::build_merkle_root(&leaves);
    println!("merkle root is {}", root);

    // build merkle proof for 42 (its index is 1);
    let proof = CBMT::build_merkle_proof(&leaves, &[0]).expect("CBMT build merkle proof failed");
    println!(
        "merkle proof lemmas are {:?}, indices are {:?}",
        proof.lemmas(),
        proof.indices(),
    );
    // verify merkle proof
    let verify_result = proof.verify(&root, &[3584654056691428718]);
    println!("merkle proof verify result is {}", verify_result);





    // build merkle proof for 42 and 20191116 (indices are 1 and 4);
    let proof = CBMT::build_merkle_proof(&leaves, &[1, 4]).expect("CBMT build merkle proof failed");
    println!(
        "merkle proof lemmas are {:?}, indices are {:?},\n ",
        proof.lemmas(),
        proof.indices()
    );
    // retrieve leaves
    let retrieved_leaves = CBMT::retrieve_leaves(&leaves, &proof);
    println!("retrieved leaves are {:?}", retrieved_leaves);
    println!(
        "calculated root of proof is {:#?}",
        proof.root(&retrieved_leaves.unwrap())
    );



   let mut hasher1 = DefaultHasher::new();
   let mut hasher2 = DefaultHasher::new();

       
        hasher1.write_u32(137123);
        hasher1.write_u32(13712);
        hasher1.finish();
        hasher1.finish();
        hasher1.write(b"Huh?");
        println!("Hash is {:x}! \n", hasher1.finish());

        let text = "Huh?".as_bytes();
        hasher2.write_u32(137123);
        hasher2.write_u32(13712);
        hasher2.finish();
        hasher2.write(text);
        println!("Hashbytes is {:x}! \n", hasher2.finish());


        use std::collections::hash_map::{DefaultHasher, RandomState};
        use std::hash::{BuildHasher, Hasher, Hash};
        
       
            let s = RandomState::new();
        
            let mut hasher = s.build_hasher();
            hasher.write(b"Cool");
            println!("Hash is {:x}", hasher.finish());
        
            let mut hasher = s.build_hasher();
            hasher.write(b"Cool");
            println!("Hash is {:x}", hasher.finish());
        
            let s = DefaultHasher::new();
        
            let mut hasher = s.clone();
            hasher.write(b"Cool");
            println!("Hash is {:x}", hasher.finish());
        
            let mut hasher = s.clone();
            hasher.write(b"Cool");
            println!("Hash is {:x}", hasher.finish());
       

}