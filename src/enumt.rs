#[macro_use]
macro_rules! test{
   ( $( $e:expr ),*)=>(
      {
      $(println!("macro testingg{}",$e);)*
      }
);

}

pub fn main(){
   test!("aldskfj" ,"alsdkj")
}
