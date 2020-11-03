use azsys;
pub use crate::az_core::*;
use std::str;
use std::slice;

//pub mod az_core;

pub struct HubClient {
    inner: azsys::az_iot_hub_client,
}

impl HubClient {

    pub fn new(host_name: &str, device_id: &str, options: Option<HubClientOptions>) -> Result<HubClient, azsys::az_result_core> {

        let options_work: *const azsys::az_iot_hub_client_options;

        match options {
            Some(o) => options_work = &o.inner,
            None => options_work = std::ptr::null(),
        }

        let mut client: HubClient = HubClient::empty_client();
        let rc = unsafe { azsys::az_iot_hub_client_init(&mut client.inner, 
            get_span_from_str(host_name),
            get_span_from_str(device_id),
            options_work
        )};

        if rc != azsys::az_result_core_AZ_OK {
            Err(rc)
        }
        else {
            Ok(client)
        }
    }

    pub fn get_client_id(&self) -> Result<String, azsys::az_result_core> {

        let mut result_work: Vec<u8> = Vec::with_capacity(100);
        let mut result_length: Vec<u64> = [ 0 ].to_vec();

        let rc = unsafe { azsys::az_iot_hub_client_get_client_id(&self.inner, 
            result_work.as_mut_ptr() as *mut i8, 
            result_work.capacity()as u64, 
            result_length.as_mut_ptr()) };

        if rc != azsys::az_result_core_AZ_OK {
            Err(rc)
        }
        else {
            unsafe { result_work.set_len(result_length[0] as usize) };
            Ok(String::from_utf8_lossy(&result_work).to_string())
        }
    }

    pub fn get_user_name(&self) -> Result<String, azsys::az_result_core> {

        let mut result_work: Vec<u8> = Vec::with_capacity(200);
        let mut result_length: Vec<u64> = [ 0 ].to_vec();

        let rc = unsafe { azsys::az_iot_hub_client_get_user_name(&self.inner, 
            result_work.as_mut_ptr() as *mut i8, 
            result_work.capacity()as u64, 
            result_length.as_mut_ptr()) };

        if rc != azsys::az_result_core_AZ_OK {
            Err(rc)
        }
        else {
            unsafe { result_work.set_len(result_length[0] as usize) };
            Ok(String::from_utf8_lossy(&result_work).to_string())
        }
    }

    pub fn get_client_telemetry_publish_topic(&self, message_properties: Option<MessageProperties>) -> Result<String, azsys::az_result> {

        let m_prop_work: *const azsys::az_iot_message_properties;

        match message_properties {
            Some(m) => m_prop_work = &m.inner,
            None => m_prop_work = std::ptr::null()
        }

        let mut result_work: Vec<u8> = Vec::with_capacity(200);
        let mut result_length: Vec<u64> = [ 0 ].to_vec();
        let rc = unsafe { azsys::az_iot_hub_client_telemetry_get_publish_topic(&self.inner,
            m_prop_work,
            result_work.as_mut_ptr() as *mut i8,
            result_work.capacity() as u64,
            result_length.as_mut_ptr()
        ) };
            
        if rc != azsys::az_result_core_AZ_OK {
            Err(rc)
        }
        else {
            unsafe { result_work.set_len(result_length[0] as usize) };
            //let test = result_work.to_str();
            Ok(String::from_utf8_lossy(&result_work).to_string())
        }
    }

    pub fn get_sas_signature(&self, ttl: u64) -> Result<Vec<u8>, azsys::az_result> {
        let mut signature_work: Vec<u8> = Vec::with_capacity(200);
        let signature = get_span_from_vector(&signature_work);
        let mut work = get_empty_span();

        let rc = unsafe { azsys::az_iot_hub_client_sas_get_signature(&self.inner, ttl, signature, &mut work) };

        if rc != azsys::az_result_core_AZ_OK {
            Err(rc)
        }
        else {
            unsafe { signature_work.set_len(get_span_size(&work) as usize) };
            Ok(signature_work)
        }
    }

    pub fn get_sas_password(&self, ttl: u64, sas: &str) -> Result<String, azsys::az_result> {
        let mut password: Vec<u8> = Vec::with_capacity(200);
        let mut length_out: Vec<u64> = [ 0 ].to_vec();
        let rc = unsafe { azsys::az_iot_hub_client_sas_get_password(&self.inner, 
                ttl,
                get_span_from_str(sas),
                get_empty_span(),
                password.as_mut_ptr() as *mut i8,
                password.capacity() as u64,
                length_out.as_mut_ptr()
            ) };
        
        if rc != azsys::az_result_core_AZ_OK {
            Err(rc)
        }
        else {
            unsafe { password.set_len(length_out[0] as usize) };
            Ok(String::from_utf8_lossy(&password).to_string())
        }
    }

    pub fn empty_client() -> HubClient {
        let client: HubClient = HubClient {
            inner: azsys::az_iot_hub_client {
                _internal: azsys::az_iot_hub_client__bindgen_ty_1 {
                    iot_hub_hostname: get_empty_span(),
                    device_id: get_empty_span(),
                    options: HubClientOptions::default_new().inner,
                }
            }
        };

        client
    }
}

pub struct HubClientOptions {
    inner: azsys::az_iot_hub_client_options,
}

impl HubClientOptions {

    pub fn default_new() -> HubClientOptions {
        HubClientOptions {
            inner: unsafe { azsys::az_iot_hub_client_options_default() }
        }
    }
}

pub struct MessageProperties {
    inner: azsys::az_iot_message_properties,
}

impl MessageProperties {
    pub fn new(buffer: Vec<u8>) -> Result<MessageProperties, azsys::az_result_core> {
        let mut message_properties: MessageProperties = MessageProperties {
            inner: azsys::az_iot_message_properties {
                _internal: azsys::az_iot_message_properties__bindgen_ty_1 {
                    properties_buffer: get_empty_span(),
                    properties_written: 0,
                    current_property_index: 0,
                }
            }
        };

        let rc = unsafe { azsys::az_iot_message_properties_init(&mut message_properties.inner, get_span_from_vector(&buffer), 0) };

        if rc != azsys::az_result_core_AZ_OK {
            Err(rc)
        }
        else {
            Ok(message_properties)
        }
    }

    pub fn append(&mut self, k: &str, v: &str) -> Result<&mut MessageProperties, azsys::az_result_core> {
        let rc = unsafe { azsys::az_iot_message_properties_append(&mut self.inner, get_span_from_str(k), get_span_from_str(v)) };

        if rc != azsys::az_result_core_AZ_OK {
            Err(rc)
        }
        else {
            Ok(self)
        }
    }

    pub fn find(&mut self, k: &str) -> Result<&str, azsys::az_result_core> {
        let mut out = get_empty_span();
        let rc = unsafe { azsys::az_iot_message_properties_find(&mut self.inner, get_span_from_str(k), &mut out) };

        if rc != azsys::az_result_core_AZ_OK {
            Err(rc)
        }
        else {
            let slice = unsafe { slice::from_raw_parts(out._internal.ptr, out._internal.size as usize) };
            // let r = unsafe {
            //     let slice = slice::from_raw_parts(out._internal.ptr, out._internal.size as usize);
            //     str::from_utf8(slice)
            // };
            Ok(str::from_utf8(slice).expect("Value is not in UTF8"))
        }
    }

    pub fn into_array(&mut self) -> Result<Vec<(&str, &str)>, std::os::raw::c_int> {

        let mut out: Vec<(&str, &str)> = Vec::new();
        let mut k = get_empty_span();
        let mut v = get_empty_span();

        loop {
            let rc = unsafe { azsys::az_iot_message_properties_next(&mut self.inner, &mut k, &mut v) as ::std::os::raw::c_int };

            if rc == azsys::az_result_core_AZ_OK as ::std::os::raw::c_int {
                let slicek = unsafe { slice::from_raw_parts(k._internal.ptr, k._internal.size as usize) };
                let slicev = unsafe { slice::from_raw_parts(v._internal.ptr, v._internal.size as usize) };
                let ks = str::from_utf8(slicek).expect("keyword is not in UTF8");
                let vs = str::from_utf8(slicev).expect("Value is not in UTF8");
                out.push((ks, vs));
                // out.push((str::from_utf8(slicek).expect("keyword is not in UTF8"), 
                //           str::from_utf8(slicev).expect("Value is not in UTF8"))).expect("Failed to store keyword/value");
            }
            else if rc == azsys::az_result_iot_AZ_ERROR_IOT_END_OF_PROPERTIES as std::os::raw::c_int {
                break;
            }
            else {
                return Err(rc);
            }
        }

        Ok(out)
    }
}
