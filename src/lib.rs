/*
 * @Author: ZuoXichen
 * @Date: 2022-12-28 15:01:41
 * @LastEditTime: 2023-01-01 14:13:33
 * @LastEditors: ZuoXichen
 * @Description: 
 */
pub mod java_remote;
pub mod java_ver;
pub mod utils;

#[no_mangle]
pub async extern "C" fn get_remote_java_version()->java_remote::JsonRelease{
    return java_remote::list_remote().await ;
}