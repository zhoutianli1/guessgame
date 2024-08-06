import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Guessgame } from "../target/types/guessgame";
import { web3 } from "@coral-xyz/anchor";
describe("guessgame", () => {
  const provider = anchor.AnchorProvider.env() //配置客户端以使用本地集群。
  anchor.setProvider(provider);//让我们连接到已配置的 Solana 开发环境 (local-test-validator)。
  

  const program = anchor.workspace.Guessgame as Program<Guessgame>;
  const seeds = Buffer.from("guessing pda2");
  //pda账户：
  const guessingPdaPubkey = anchor.web3.PublicKey.findProgramAddressSync(
    [seeds],
    program.programId  //程序 的程序 ID。
  );
  const guessingAccount = anchor.web3.Keypair.generate();
  async function initialize() {
    try {
      const initializeTx = await program.methods
        .initialize()
        .accounts({
          guessing_account:guessingPdaPubkey[0],
          payer: provider.wallet.publicKey,
          systemProgram: web3.SystemProgram.programId,
        })
        .rpc();//该 rpc() 方法 使用指定的指令发送已签名的交易 并返回TransactionSignature。使用 时 .rpc， Wallet 的Provider会自动包含在签名者中。
      //const account = await program.account.guessingAccount.fetch(guessingPdaPubkey[0]);
      console.log(
        "初始化成功！\n您的交易签名是:",
        initializeTx
      );
    
      
    } catch (errors: any) {
      console.log("错误");
    }
  }
  
  async function guessing(number: number) {
    try {
      const guessingTx = await program.methods
        .guess(number)
        .accounts({
          guessing_account: guessingPdaPubkey[0],
          payer: provider.wallet.publicKey,
          systemProgram: web3.SystemProgram.programId,
        })
        .rpc();
      
      console.log("恭喜你答对了！");
      console.log(
        "账户:",
        guessingPdaPubkey
      );
    
      
    } catch (errors: any) {
      console.log("报错：");
      console.log(errors.error.errorMessage);

    }
  }
  
  initialize();
  //guessing(0);
});

//