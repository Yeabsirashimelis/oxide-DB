/*
   ByteStr is to &str what ByteString is to Vec<u8>.

   This code processes lots of Vec<u8> data. Because that is used in the same way as String tends to be used,
    ByteString is a useful alias.
*/

type ByteString = Vec<u8>;

type ByteStr = [u8];
