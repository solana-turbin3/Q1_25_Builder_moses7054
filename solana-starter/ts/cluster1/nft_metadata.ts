import wallet from "../../id.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createGenericFile,
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";

// Create a devnet connection
const umi = createUmi("https://api.devnet.solana.com");

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
  try {
    // Follow this JSON structure
    // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

    const image =
      "https://arweave.net/3AuT6zSdXsvWMktvWLp4e9iVUNiEzXyCW78cuc4aLG5q";
    const metadata = {
      name: "The Rug",
      symbol: "",
      description: "the rug",
      image,
      attributes: [{ trait_type: "sd", value: "sd" }],
      properties: {
        files: [
          {
            type: "image/png",
            uri: "https://arweave.net/3AuT6zSdXsvWMktvWLp4e9iVUNiEzXyCW78cuc4aLG5q",
          },
        ],
      },
      creators: [],
    };
    const myUri = await umi.uploader.uploadJson(metadata);
    console.log("Your metadata URI: ", myUri);
  } catch (error) {
    console.log("Oops.. Something went wrong", error);
  }
})();


// Your metadata URI:  https://arweave.net/2TRNDbkuJibMgrYZiZubAD9CoACuNimdxYUqgTPYM4xr