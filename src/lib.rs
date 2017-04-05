use std::io;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    println!("{:?}", "hello");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub struct Locator {
	ipData : Vec<u8>,
	textOffset : u32,
	index : [u32;256],
	indexData1 : Vec<u32>,
	indexData2 : Vec<u32>,
	indexData3 : Vec<u8>,
}

pub struct LocationInfo {
	pub country : String,
    pub state : String,
    pub city : String,
    pub isp : String,
}

pub trait ILocator {
    fn findStr(&self, ip:&str) -> &LocationInfo;

    fn findBytes(&self, ip:&[u8]) -> &LocationInfo;

    fn findInt(&self, ip:u32) -> &LocationInfo;
}

impl Locator {
    // todo check valid data
    pub fn load_ipdb(data : &str) -> std::io::Result<Locator>{
    	let ipData = data.as_bytes();
    	let textOffset = bigEndian(data, 0);
    	let index:[u32;256] = [0, 256];
    	let mut i = 0;
    	
    	while i < 256 {
    	    index[i] = littleEndian(data, 4 + i * 4);
    	    i+=1;
    	}
    	
    	let nidx = (textOffset - 4 - 1024 - 1024) / 8;

    	let indexData1:[u32] = [0, nidx];
    	let indexData2:[u32] = [0, nidx];
    	let indexData3:[u8] = [0, nidx];

    	i = 0;
    	let mut off = 0;
     	while i < nidx {
     	    off = 4 + 1024 + i * 8;
     	    indexData1[i] = bigEndian(ipData, off);
            indexData2[i] = ((ipData[off + 6] & 0xff) << 16) + (( ipData[off + 5] & 0xff) << 8) + ( ipData[off + 4] & 0xff);
            indexData3[i] = ipData[off + 7];
            i+=1;
     	}

     	return Locator{
     		ipData: ipData, 
     		textOffset:textOffset, 
     		index:index, 
     		indexData1:indexData1, 
     		indexData2:indexData2, 
     		indexData3:indexData3
     	};
    }

    pub fn load_ipdb_file(path : &str) -> std::io::Result<Locator> {
    	let path = Path::new(path);

    	// Open the path in read-only mode, returns `io::Result<File>`
    	let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        	Err(why) => return Err(why),
        	Ok(file) => file,
    	};

    	let mut s = String::new();
    	match file.read_to_string(&mut s) {
        	Err(why) => return Err(why),
    	}

    	return Self::load_ipdb(s);
    }
}

impl ILocator for Locator {
    fn findStr(&self, ip:&str) -> &LocationInfo{

    }

    fn findBytes(&self, ip:&[u8]) -> &LocationInfo{

    }

    fn findInt(&self, ip:u32) -> &LocationInfo{

    }
}

fn bigEndian(data : &[u8], offset:u32) -> u32{
	let a:u32 = data[offset] & 0xff;
	let b:u32 = data[offset+1] & 0xff;
	let c:u32 = data[offset+2] & 0xff;
	let d:u32 = data[offset+3] & 0xff;
    
    return (a << 24) | (b << 16) | (c << 8) | d;
 }

 fn littleEndian(data : &[u8], offset:u32) -> u32{
    let a:u32 = data[offset] & 0xff;
	let b:u32 = data[offset+1] & 0xff;
	let c:u32 = data[offset+2] & 0xff;
	let d:u32 = data[offset+3] & 0xff;
    return (d << 24) | (c << 16) | (b << 8) | a;
 }

 // fn parseOctet(ipPart:&str) -> u8{
 // 	// let octet:u32 = 
 //        // Note: we already verified that this string contains only hex digits.
 //        int octet = Integer.parseInt(ipPart);
 //        // Disallow leading zeroes, because no clear standard exists on
 //        // whether these should be interpreted as decimal or octal.
 //        if (octet < 0 || octet > 255 || (ipPart.startsWith("0") && ipPart.length() > 1)) {
 //            throw new NumberFormatException("invalid ip part");
 //        }
 //        return (byte) octet;
 // }

 // fn textToNumericFormatV4(String str) -> [u8]{
 //        String[] s = str.split("\\.");
 //        if (s.length != 4) {
 //            throw new NumberFormatException("the ip is not v4");
 //        }
 //        byte[] b = new byte[4];
 //        b[0] = parseOctet(s[0]);
 //        b[1] = parseOctet(s[1]);
 //        b[2] = parseOctet(s[2]);
 //        b[3] = parseOctet(s[3]);
 //        return b;
 // }
