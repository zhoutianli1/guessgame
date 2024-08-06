use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;
use std::cmp::Ordering;

//每个程序都有一个唯一的地址，称为程序 ID（programId）。我们使用 declared_id !宏来指定程序的链上地址。
//当你第一次构建 Anchor 程序时，框架会生成一个新的【密钥对】。这个密钥对的【公钥】将成为你的程序 ID。;
declare_id!("4ggHANFvbtRT3XDagtP2GGLvZk4pH7paewKZUVMABg9w");

#[program]
pub mod guessgame {//pub模块，可以保证其中的函数（后面会编写的随机数初始化和玩家猜谜函数）可以被外部访问和使用
    use super::*;//从父模块导入内容

 //指令1:初始化指令。用于：初始化一个随机数
    //tx ：接受Context<AccountContext>类型的参数，表示当前程序上下文。
    //Result<()> ：返回一个Result类型，其中()表示成功时不返回任何值，而错误情况将返回Err 。
    pub fn initialize(ctx: Context<AccountContext>) -> Result<()> {  
        //这行代码获取了一个可变引用
        let guessing_account = &mut ctx.accounts.guessing_account; //获取账户1，AccountContext实例，玩家猜测的数字
        //为游戏生成一个随机数字
        guessing_account.number = generate_number();
        
        Ok(())
    }

    //指令2:玩家猜数
    pub fn guess(ctx: Context<AccountContext>, number: u32) -> Result<()> {
        let guessing_account: &mut Account<GuessingAccount> = &mut ctx.accounts.guessing_account;//accounts是从客户端传入的账户组
        let target = guessing_account.number;
        match number.cmp(&target) {
            Ordering::Less => {
                return err!(MyError::NumberTooSmall)
            }
            Ordering::Greater => {
                return  err!(MyError::NumberTooLarge);
            }
            Ordering::Equal => {
                return Ok(())
            }
        }
    }
}

#[error_code]
pub enum MyError {
    #[msg("Too small")]
    NumberTooSmall,
    #[msg("Too larget")]
    NumberTooLarge
}


//函数：生成随机数。我们将从 Solana 程序库中获取当前时间，并使用该时间生成随机数。
fn generate_number()-> u32{
    //从 Clock模块来获取区块链状态下的当前时间信息：
    let clock = Clock::get().expect("获取时间失败"); //时钟对象
    //获取时间戳，取模到最后一位
    let last_digit = (clock.unix_timestamp %10) as u32; //0到9
    last_digit
}

//#[derive(Accounts)]宏：定义：指令中程序账户结构-Context，包含执行智能合约所需的所有必要的账户引用。
//用于存储和操作游戏状态；
//指令如何访问：Context.accounts ；指令中一般用ctx代替Context，ctx.accounts
#[derive(Accounts)]
pub struct AccountContext<'info> 
{   
    //'info ：一个生命周期参数，确保此帐户引用在整个AccountContext结构的生命周期内有效。
    //用于传递玩家的猜测。
    #[account(          //#[account]宏来配置 guessing_account的各项属性，比如初始化方法，空间大小，付款人账户等。
        init, //指示 Anchor 在需要时自动初始化帐户。如果帐户尚未初始化，Anchor 将根据提供的其他参数（如空间和付款人）对其进行初始化。
        space=32,       //账户数据所需的存储空间大小，
        payer=payer,    //指定付款人账户。为下面用户
        seeds = [b"guessing pda"], //用于生成程序派生地址（PDA）。
        bump
    )]
    //账户1:
    pub guessing_account: Account<'info, GuessingAccount>,  //GuessingAccount指定此帐户将保存的数据类型

    //账户2:添加付款人字段：表示执行智能合约所需的付款人账户。
    #[account(mut)]  //#[account(mut)] ：表示付款人是一个可变的账户引用，因为账户状态可能会在合约执行期间被修改（例如，扣除费用）。
    pub payer: Signer<'info>,  //付款人属于Signer类型，代表进行交易的实体。

    //账户3:系统账户。表示对 Solana 系统程序的引用，它提供了合约执行所需的一些基本功能。
    pub system_program: Program<'info, System>,
}




//#[account] ：用于定义程序的自定义【数据帐户】类型的格式，使得结构能够映射到区块链上的帐户，存储所需的状态信息，并通过合约中的函数进行访问和修改
#[account] 
pub struct GuessingAccount {
    pub number: u32 //记录玩家的猜测结果
}