use aes::Aes128;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};
    fn main() {

        let mut array: [u8; 16] = [0;16];
        let mut array2: [u8; 16] = [0;16];
        let mut i =0;

        //Remplissage du tableau à chiffrer
        for c in "1234567891234567891".chars() {
            println!("{}", c);
            let my_string = c.to_string();  
            let my_int = my_string.parse::<u8>().unwrap(); 
            if i > 15{
                array2[i-16]=my_int;
            }
            else {
                array[i]=my_int;
            }
            i+=1;
        }
        println!("{}{}{}", array[0],array[1],array[2]);
        println!("{}{}{}", array2[0],array2[1],array2[2]);

        // Définition de la clé et des blocks
        let key = GenericArray::from([10,3,5,1,3,5,1,3,5,1,3,5,1,3,5,6]);
        let mut block = GenericArray::from(array);
        let mut block2=GenericArray::from(array2);

        println!("Clé AES: {:?}",key);
        println!("Block clair: {:?}",block);
        println!("Block clair 2: {:?}",block2);
        //Composants pour chiffrer 

        let cipher = Aes128::new(&key);
 
        let block_copy = block.clone();
        let block_copy2 = block2.clone();
    
        cipher.encrypt_block(&mut block);
        cipher.encrypt_block(&mut block2);

        println!("Block 0: {:?}, Block 1 {}, Block_Copy {}", block[0], block[1], block_copy[0]);
        println!("Block2 0: {:?}, Block2 1 {}, Block_Copy2 {}", block2[0], block2[1], block_copy2[0]);

        cipher.decrypt_block(&mut block);
        cipher.decrypt_block(&mut block2);

        assert_eq!(block, block_copy);

        println!("Déchiffrement");
        println!("Block 0: {:?}, Block 1 {}, Block_copy {}", block[0], block[1], block_copy[0]);
        println!("Block2 0: {:?}, Block2 1 {}, Block_copy2 {}", block[0], block[1], block_copy[0]);
        
    }
