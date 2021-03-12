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

      // 转换 Leaves[&Str] 节点数组 到 leaves[HASHS]
    let leaves: Vec<_> =  leaves_1.iter().map(|x| { 
        let mut hasher = DefaultHasher::new();
        hasher.write(x.as_bytes());
        hasher.finish()
    }).collect();


    println!("leaves convert from string: {:#?} => \n u64 : {:#?} ", leaves_1, leaves );

     // 计算 mkroot
    let root = CBMT::build_merkle_root(&leaves);
    println!("merkle root is {}", root);

    // build merkle proof for 42 (its index is 2);
      // 计算 [merkle proof] for 42 (its index is 2);
    let proof = CBMT::build_merkle_proof(&leaves, &[2]).expect("CBMT build merkle proof failed");
    println!(
        "merkle proof [2] lemmas are {:?}, indices are {:?}",
        proof.lemmas(),
        proof.indices(),
    );

    let myu64:u64 = 12334026276191850359;
    let my_string = myu64.to_string();  // `parse()` works with `&str` and `String`!

    println!("---------myu64 convert to string  is  {}", my_string);
     let my_u64:u64 = my_string.parse().expect("string convert to u64 error");
     println!("---------mystring convert to u64 is  {}", my_u64);
 
    // verify merkle proof
    let verify_result = proof.verify(&root, &[12334026276191850359]);
    println!("---------merkle proof verify result is {}", verify_result);

    let a = ["SDFSDF", "FDGDFGDF", "23423", "56546", "345345345"];

    for (i, v) in a.iter().enumerate() {
            println!("elem at index {} is {}", i,v);
    }



    /*  // rebuild proof
        
    fn rebuild_proof() {
        let leaves = vec![2i32, 3, 5, 7, 11];
        let tree = CBMTI32::build_merkle_tree(&leaves);
        let root = tree.root();

        // build proof
        let proof = tree.build_proof(&[0, 3]).unwrap();
        let lemmas = proof.lemmas();
        let indices = proof.indices();

        // rebuild proof
        let needed_leaves: Vec<i32> = indices
            .iter()
            .map(|i| tree.nodes()[*i as usize].clone())
            .collect();
        let rebuild_proof = CBMTI32Proof::new(indices.to_vec(), lemmas.to_vec());    // 用 indices + lemmas 重建 rproof
        assert_eq!(rebuild_proof.verify(&root, &needed_leaves), true);    // 用 root 和 indics-leaves 来验证 rproof                                         
        assert_eq!(root, rebuild_proof.root(&needed_leaves).unwrap());    // 说明需要 4个属性   1. indices  2. lemmas[]，3. root 4. indics-leaves
    }

    */



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
        use std::hash::{BuildHasher, Hash};
        
       
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