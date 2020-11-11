pub use crate::az_core::*;
pub use crate::az_return_codes::AzReturnCode;
use azsys;
use std::slice;
use std::str;

pub struct HubClientBuilder<'a> {
    host_name: Option<&'a str>,
    device_id: Option<&'a str>,
    client_options: Option<HubClientOptions>,
}

pub struct HubClient {
    inner: azsys::az_iot_hub_client,
}

impl<'a> HubClientBuilder<'a> {
    pub fn new() -> HubClientBuilder<'static> {
        HubClientBuilder {
            host_name: Option::None,
            device_id: Option::None,
            client_options: Option::None,
        }
    }

    pub fn host_name(&mut self, host_name: &'a str) -> &mut HubClientBuilder<'a> {
        self.host_name = Option::Some(&host_name);
        self
    }

    pub fn device_id(&mut self, device_id: &'a str) -> &mut HubClientBuilder<'a> {
        self.device_id = Option::Some(device_id);
        self
    }

    pub fn client_options(
        &mut self,
        client_options: HubClientOptions,
    ) -> &mut HubClientBuilder<'a> {
        self.client_options = Option::Some(client_options);
        self
    }

    pub fn finalize(&mut self) -> Result<HubClient, AzReturnCode> {
        if None == self.host_name || None == self.device_id {
            panic!("Missing required parameters");
        }

        let options_work: *const azsys::az_iot_hub_client_options;

        match &self.client_options {
            Some(o) => options_work = &o.inner,
            None => options_work = std::ptr::null(),
        }

        let mut result = HubClient::new_empty();
        let rc = unsafe {
            azsys::az_iot_hub_client_init(
                &mut result.inner,
                get_span_from_str(&self.host_name.as_ref().unwrap()),
                get_span_from_str(&self.device_id.as_ref().unwrap()),
                options_work,
            )
        };

        if rc != azsys::az_result_core_AZ_OK {
            Err(AzReturnCode::from_i32(rc))
        } else {
            Ok(result)
        }
    }
}

pub enum TopicType {
    C2D(ClientC2DRequest),
    Method(ClientMethodRequest),
    Unknown,
}

impl HubClient {
    pub fn new(
        host_name: &str,
        device_id: &str,
        options: Option<HubClientOptions>,
    ) -> Result<HubClient, AzReturnCode> {
        let options_work: *const azsys::az_iot_hub_client_options;

        match options {
            Some(o) => options_work = &o.inner,
            None => options_work = std::ptr::null(),
        }

        let mut client: HubClient = HubClient::new_empty();
        let rc = unsafe {
            azsys::az_iot_hub_client_init(
                &mut client.inner,
                get_span_from_str(host_name),
                get_span_from_str(device_id),
                options_work,
            )
        };

        if rc != azsys::az_result_core_AZ_OK {
            Err(AzReturnCode::from_i32(rc))
        } else {
            Ok(client)
        }
    }

    pub fn new_empty() -> HubClient {
        HubClient {
            inner: azsys::az_iot_hub_client {
                _internal: azsys::az_iot_hub_client__bindgen_ty_1 {
                    iot_hub_hostname: get_empty_span(),
                    device_id: get_empty_span(),
                    options: HubClientOptions::default_new().inner,
                },
            },
        }
    }

    pub fn get_client_id(&self) -> Result<String, AzReturnCode> {
        let mut capacity: usize = 100;
        let mut result = String::with_capacity(capacity);

        loop {
            let rc = self.ll_get_client_id(&mut result);

            match rc {
                AzReturnCode::AzResultCoreErrorNotEnoughSpace => {
                    capacity *= 2;
                    result = String::with_capacity(capacity);
                    continue;
                }
                AzReturnCode::AzResultCoreOk => {
                    result.shrink_to_fit();
                    return Ok(result);
                }
                _ => {
                    return Err(rc);
                }
            }
        }
    }

    pub fn ll_get_client_id(&self, result: &mut String) -> AzReturnCode {
        let mut len: u64 = 0;
        let len_ptr: *mut u64 = &mut len;

        let rc = unsafe {
            azsys::az_iot_hub_client_get_client_id(
                &self.inner,
                result.as_mut_vec().as_mut_ptr() as *mut i8,
                result.capacity() as u64,
                len_ptr,
            )
        };

        if rc == azsys::az_result_core_AZ_OK {
            unsafe { result.as_mut_vec().set_len(len as usize) };
        }

        AzReturnCode::from_i32(rc)
    }

    pub fn get_user_name(&self) -> Result<String, AzReturnCode> {
        let mut capacity: usize = 100;
        let mut result = String::with_capacity(capacity);

        loop {
            let rc = self.ll_get_user_name(&mut result);

            match rc {
                AzReturnCode::AzResultCoreErrorNotEnoughSpace => {
                    capacity *= 2;
                    result = String::with_capacity(capacity);
                    continue;
                }
                AzReturnCode::AzResultCoreOk => {
                    result.shrink_to_fit();
                    return Ok(result);
                }
                _ => {
                    return Err(rc);
                }
            }
        }
    }

    pub fn ll_get_user_name(&self, result: &mut String) -> AzReturnCode {
        let mut len: u64 = 0;
        let len_ptr: *mut u64 = &mut len;

        let rc = unsafe {
            azsys::az_iot_hub_client_get_user_name(
                &self.inner,
                result.as_mut_vec().as_mut_ptr() as *mut i8,
                result.capacity() as u64,
                len_ptr,
            )
        };

        if rc == azsys::az_result_core_AZ_OK {
            unsafe { result.as_mut_vec().set_len(len as usize) };
        }

        AzReturnCode::from_i32(rc)
    }

    pub fn get_c2d_subscribe_topic() -> &'static str {
        static AZ_IOT_HUB_CLIENT_C2D_SUBSCRIBE_TOPIC: &str = "devices/+/messages/devicebound/#";
        AZ_IOT_HUB_CLIENT_C2D_SUBSCRIBE_TOPIC
    }

    pub fn c2d_parse_received_topic(&self, topic: &str) -> Result<ClientC2DRequest, AzReturnCode> {
        let mut result: ClientC2DRequest = ClientC2DRequest::new_empty();
        let rc = unsafe {
            azsys::az_iot_hub_client_c2d_parse_received_topic(
                &self.inner,
                get_span_from_str(topic),
                &mut result.inner,
            )
        };

        if rc != azsys::az_result_core_AZ_OK {
            Err(AzReturnCode::from_i32(rc))
        } else {
            Ok(result)
        }
    }

    pub fn get_method_subscribe_topic() -> &'static str {
        static AZ_IOT_HUB_CLIENT_METHODS_SUBSCRIBE_TOPIC: &str = "$iothub/methods/POST/#";
        AZ_IOT_HUB_CLIENT_METHODS_SUBSCRIBE_TOPIC
    }

    pub fn methods_parse_received_topic(
        &self,
        topic: &str,
    ) -> Result<ClientMethodRequest, AzReturnCode> {
        let mut result: ClientMethodRequest = ClientMethodRequest::new_empty();
        let rc = unsafe {
            azsys::az_iot_hub_client_methods_parse_received_topic(
                &self.inner,
                get_span_from_str(topic),
                &mut result.inner,
            )
        };

        if rc != azsys::az_result_core_AZ_OK {
            Err(AzReturnCode::from_i32(rc))
        } else {
            Ok(result)
        }
    }

    pub fn methods_response_get_publish_topic(&self, request_id: &str, status: u16) -> Result<String, AzReturnCode> {
        let mut capacity: usize = 100;
        let mut result = String::with_capacity(capacity);

        loop {
            let rc = self.ll_methods_response_get_publish_topic(&request_id, status, &mut result);

            match rc {
                AzReturnCode::AzResultCoreErrorNotEnoughSpace => {
                    capacity *= 2;
                    result = String::with_capacity(capacity);
                    continue;
                }
                AzReturnCode::AzResultCoreOk => {
                    result.shrink_to_fit();
                    return Ok(result);
                }
                _ => {
                    return Err(rc);
                }
            }
        }
    }

    pub fn ll_methods_response_get_publish_topic(&self, request_id: &str, status: u16, result: &mut String) -> AzReturnCode {
        let request_id_span = get_span_from_str(request_id);
        let mut len: u64 = 0;
        let len_ptr: *mut u64 = &mut len;
        let rc = unsafe { azsys::az_iot_hub_client_methods_response_get_publish_topic(
            &self.inner, 
            request_id_span, 
            status,                 
            result.as_mut_vec().as_mut_ptr() as *mut i8,
            result.capacity() as u64,
            len_ptr,
        ) };

        if rc == azsys::az_result_core_AZ_OK {
            unsafe { result.as_mut_vec().set_len(len as usize) };
        }

        AzReturnCode::from_i32(rc)
    }

    pub fn get_twin_respnse_subscribe_topic() -> &'static str {
        static AZ_IOT_HUB_CLIENT_TWIN_RESPONSE_SUBSCRIBE_TOPIC: &str = "$iothub/twin/res/#";
        AZ_IOT_HUB_CLIENT_TWIN_RESPONSE_SUBSCRIBE_TOPIC
    }

    pub fn get_twin_patch_subscribe_topic() -> &'static str {
        static AZ_IOT_HUB_CLIENT_TWIN_PATCH_SUBSCRIBE_TOPIC: &str =
            "$iothub/twin/PATCH/properties/desired/#";
        AZ_IOT_HUB_CLIENT_TWIN_PATCH_SUBSCRIBE_TOPIC
    }

    pub fn get_topic_type(&self, topic: &str) -> Result<TopicType, AzReturnCode> {
        match self.c2d_parse_received_topic(topic) {
            Ok(val) =>  { return Ok(TopicType::C2D(val)); },
            Err(rc) => {
                if rc != AzReturnCode::AzResultIoTErrorTopicNoMatch {
                    return Err(rc);
                }
            },
        }

        match self.methods_parse_received_topic(topic) {
            Ok(val) => { return Ok(TopicType::Method(val)); },
            Err(rc) => {
                if rc != AzReturnCode::AzResultIoTErrorTopicNoMatch {
                    return Err(rc);
                }
            },
        }

        Ok(TopicType::Unknown)
    }

    pub fn get_telemetry_publish_topic(
        &self,
        message_properties: Option<MessageProperties>,
    ) -> Result<String, AzReturnCode> {
        let mut capacity: usize = 100;
        let mut result = String::with_capacity(capacity);

        loop {
            let rc = self.ll_get_telemetry_publish_topic(&message_properties, &mut result);

            match rc {
                AzReturnCode::AzResultCoreErrorNotEnoughSpace => {
                    capacity *= 2;
                    result = String::with_capacity(capacity);
                    continue;
                }
                AzReturnCode::AzResultCoreOk => {
                    result.shrink_to_fit();
                    return Ok(result);
                }
                _ => {
                    return Err(rc);
                }
            }
        }
    }

    pub fn ll_get_telemetry_publish_topic(
        &self,
        message_properties: &Option<MessageProperties>,
        result: &mut String,
    ) -> AzReturnCode {
        let mut len: u64 = 0;
        let len_ptr: *mut u64 = &mut len;
        let m_prop_work: *const azsys::az_iot_message_properties = match message_properties {
            Some(m) => &m.inner,
            None => std::ptr::null(),
        };
        let rc = unsafe {
            azsys::az_iot_hub_client_telemetry_get_publish_topic(
                &self.inner,
                m_prop_work,
                result.as_mut_vec().as_mut_ptr() as *mut i8,
                result.capacity() as u64,
                len_ptr,
            )
        };

        if rc == azsys::az_result_core_AZ_OK {
            unsafe { result.as_mut_vec().set_len(len as usize) };
        }

        AzReturnCode::from_i32(rc)
    }

    pub fn get_sas_signature(&self, ttl: u64) -> Result<Vec<u8>, AzReturnCode> {
        let mut capacity: usize = 200;
        let mut result: Vec<u8> = Vec::with_capacity(capacity);

        loop {
            let rc = self.ll_get_sas_signature(ttl, &mut result);

            match rc {
                AzReturnCode::AzResultCoreErrorNotEnoughSpace => {
                    capacity *= 2;
                    result = Vec::with_capacity(capacity);
                    continue;
                }
                AzReturnCode::AzResultCoreOk => {
                    result.shrink_to_fit();
                    return Ok(result);
                }
                _ => {
                    return Err(rc);
                }
            }
        }
    }

    pub fn ll_get_sas_signature(&self, ttl: u64, result: &mut Vec<u8>) -> AzReturnCode {
        let result_span = get_span_from_vector(&result);
        let mut work = get_empty_span();
        let rc = unsafe {
            azsys::az_iot_hub_client_sas_get_signature(&self.inner, ttl, result_span, &mut work)
        };

        if rc == azsys::az_result_core_AZ_OK {
            unsafe { result.set_len(get_span_size(&work) as usize) };
        }

        AzReturnCode::from_i32(rc)
    }

    pub fn get_sas_password(&self, ttl: u64, sas: &str) -> Result<String, AzReturnCode> {
        let mut capacity: usize = 300;
        let mut result = String::with_capacity(capacity);

        loop {
            let rc = self.ll_get_sas_password(ttl, sas, &mut result);

            match rc {
                AzReturnCode::AzResultCoreErrorNotEnoughSpace => {
                    capacity *= 2;
                    result = String::with_capacity(capacity);
                    continue;
                }
                AzReturnCode::AzResultCoreOk => {
                    result.shrink_to_fit();
                    return Ok(result);
                }
                _ => {
                    return Err(rc);
                }
            }
        }
    }

    pub fn ll_get_sas_password(&self, ttl: u64, sas: &str, result: &mut String) -> AzReturnCode {
        // TODO: Add key_name option
        let mut len: u64 = 0;
        let len_ptr: *mut u64 = &mut len;
        let rc = unsafe {
            azsys::az_iot_hub_client_sas_get_password(
                &self.inner,
                ttl,
                get_span_from_str(sas),
                get_empty_span(),
                result.as_mut_ptr() as *mut i8,
                result.capacity() as u64,
                len_ptr,
            )
        };

        if rc == azsys::az_result_core_AZ_OK {
            unsafe { result.as_mut_vec().set_len(len as usize) };
        }

        AzReturnCode::from_i32(rc)
    }

    pub fn calculate_retry_delay(
        operation_msec: i32, 
        attempt: i16, 
        min_retry_delay_msec: i32,
        max_retry_delay_msec: i32,
        jitter: i32,
    ) -> i32 {
        unsafe { azsys::az_iot_calculate_retry_delay(operation_msec, attempt, min_retry_delay_msec, max_retry_delay_msec, jitter) }
    }
}

pub struct HubClientOptions {
    inner: azsys::az_iot_hub_client_options,
}

impl HubClientOptions {
    pub fn default_new() -> HubClientOptions {
        HubClientOptions {
            inner: unsafe { azsys::az_iot_hub_client_options_default() },
        }
    }
}

pub struct MessageProperties {
    inner: azsys::az_iot_message_properties,
}

impl MessageProperties {
    pub fn new(buffer: Vec<u8>) -> Result<MessageProperties, AzReturnCode> {
        let mut message_properties = MessageProperties::new_empty();
        let rc = unsafe {
            azsys::az_iot_message_properties_init(
                &mut message_properties.inner,
                get_span_from_vector(&buffer),
                0,
            )
        };

        if rc != azsys::az_result_core_AZ_OK {
            Err(AzReturnCode::from_i32(rc))
        } else {
            Ok(message_properties)
        }
    }

    pub fn new_empty() -> MessageProperties {
        let message_properties: MessageProperties = MessageProperties {
            inner: azsys::az_iot_message_properties {
                _internal: azsys::az_iot_message_properties__bindgen_ty_1 {
                    properties_buffer: get_empty_span(),
                    properties_written: 0,
                    current_property_index: 0,
                },
            },
        };

        message_properties
    }

    pub fn append(&mut self, k: &str, v: &str) -> Result<&mut MessageProperties, AzReturnCode> {
        let rc = unsafe {
            azsys::az_iot_message_properties_append(
                &mut self.inner,
                get_span_from_str(k),
                get_span_from_str(v),
            )
        };

        if rc != azsys::az_result_core_AZ_OK {
            Err(AzReturnCode::from_i32(rc))
        } else {
            Ok(self)
        }
    }

    pub fn find(&mut self, k: &str) -> Result<&str, AzReturnCode> {
        let mut out = get_empty_span();
        let rc = unsafe {
            azsys::az_iot_message_properties_find(&mut self.inner, get_span_from_str(k), &mut out)
        };

        if rc != azsys::az_result_core_AZ_OK {
            Err(AzReturnCode::from_i32(rc))
        } else {
            let slice =
                unsafe { slice::from_raw_parts(out._internal.ptr, out._internal.size as usize) };
            // let r = unsafe {
            //     let slice = slice::from_raw_parts(out._internal.ptr, out._internal.size as usize);
            //     str::from_utf8(slice)
            // };
            Ok(str::from_utf8(slice).expect("Value is not in UTF8"))
        }
    }

    pub fn into_array(&mut self) -> Result<Vec<(&str, &str)>, AzReturnCode> {
        let mut out: Vec<(&str, &str)> = Vec::new();
        let mut k = get_empty_span();
        let mut v = get_empty_span();

        loop {
            let rc = unsafe {
                azsys::az_iot_message_properties_next(&mut self.inner, &mut k, &mut v)
                    as ::std::os::raw::c_int
            };

            if rc == azsys::az_result_core_AZ_OK {
                let slicek =
                    unsafe { slice::from_raw_parts(k._internal.ptr, k._internal.size as usize) };
                let slicev =
                    unsafe { slice::from_raw_parts(v._internal.ptr, v._internal.size as usize) };
                let ks = str::from_utf8(slicek).expect("keyword is not in UTF8");
                let vs = str::from_utf8(slicev).expect("Value is not in UTF8");
                out.push((ks, vs));
            } else if rc == azsys::az_result_iot_AZ_ERROR_IOT_END_OF_PROPERTIES {
                break;
            } else {
                return Err(AzReturnCode::from_i32(rc));
            }
        }

        Ok(out)
    }
}

pub struct ClientC2DRequest {
    inner: azsys::az_iot_hub_client_c2d_request,
}

impl ClientC2DRequest {
    pub fn new_empty() -> ClientC2DRequest {
        let result: ClientC2DRequest = ClientC2DRequest {
            inner: azsys::az_iot_hub_client_c2d_request {
                properties: azsys::az_iot_message_properties {
                    _internal: azsys::az_iot_message_properties__bindgen_ty_1 {
                        properties_buffer: get_empty_span(),
                        properties_written: 0,
                        current_property_index: 0,
                    },
                },
            },
        };

        result
    }

    pub fn get_message_properties(&self) -> MessageProperties {
        let result: MessageProperties = MessageProperties {
            inner: azsys::az_iot_message_properties {
                _internal: azsys::az_iot_message_properties__bindgen_ty_1 {
                    properties_buffer: self.inner.properties._internal.properties_buffer,
                    properties_written: self.inner.properties._internal.properties_written,
                    current_property_index: self.inner.properties._internal.current_property_index,
                },
            },
        };

        result
    }
}

pub struct ClientMethodRequest {
    inner: azsys::az_iot_hub_client_method_request,
}

impl ClientMethodRequest {
    pub fn new_empty() -> ClientMethodRequest {
        ClientMethodRequest {
            inner: azsys::az_iot_hub_client_method_request {
                request_id: get_empty_span(),
                name: get_empty_span(),
            },
        }
    }

    pub fn get_request_id(&self) -> &str {
        let slice = unsafe {
            slice::from_raw_parts(
                get_span_ptr(&self.inner.request_id),
                get_span_size(&self.inner.request_id) as usize,
            )
        };
        str::from_utf8(&slice).expect("Request Id contains unprintable characters")
    }

    pub fn get_name(&self) -> &str {
        let slice = unsafe {
            slice::from_raw_parts(
                get_span_ptr(&self.inner.name),
                get_span_size(&self.inner.name) as usize,
            )
        };
        str::from_utf8(&slice).expect("Request Id contains unprintable characters")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static HOST_NAME: &str = "testhost.azure-devices.net";
    static DEVICE_ID: &str = "test1";
    #[test]
    fn client_init() {
        let client = HubClient::new(HOST_NAME, DEVICE_ID, Option::None).unwrap();
        let test: String = unsafe {
            String::from_raw_parts(
                client.inner._internal.iot_hub_hostname._internal.ptr,
                client.inner._internal.iot_hub_hostname._internal.size as usize,
                client.inner._internal.iot_hub_hostname._internal.size as usize,
            )
        };
        assert_eq!(HOST_NAME, test);
        let _test = std::mem::ManuallyDrop::new(test);
        let test: String = unsafe {
            String::from_raw_parts(
                client.inner._internal.device_id._internal.ptr,
                client.inner._internal.device_id._internal.size as usize,
                client.inner._internal.device_id._internal.size as usize,
            )
        };
        assert_eq!(DEVICE_ID, test);
        let _test = std::mem::ManuallyDrop::new(test);
    }
    #[test]
    fn client_builder() {
        let client = HubClientBuilder::new()
            .host_name(&HOST_NAME)
            .device_id(&DEVICE_ID)
            .finalize()
            .unwrap();
        let test: String = unsafe {
            String::from_raw_parts(
                client.inner._internal.iot_hub_hostname._internal.ptr,
                client.inner._internal.iot_hub_hostname._internal.size as usize,
                client.inner._internal.iot_hub_hostname._internal.size as usize,
            )
        };
        assert_eq!(HOST_NAME, test);
        let _test = std::mem::ManuallyDrop::new(test);
        let test: String = unsafe {
            String::from_raw_parts(
                client.inner._internal.device_id._internal.ptr,
                client.inner._internal.device_id._internal.size as usize,
                client.inner._internal.device_id._internal.size as usize,
            )
        };
        assert_eq!(DEVICE_ID, test);
        let _test = std::mem::ManuallyDrop::new(test);
    }
    #[test]
    fn client_get_client_id() {
        let client = HubClient::new(HOST_NAME, DEVICE_ID, Option::None).unwrap();
        let output = client.get_client_id().unwrap();
        assert_eq!(DEVICE_ID, output);
        assert_eq!(output.len(), output.capacity());
    }
    #[test]
    fn client_ll_get_client_id() {
        let client = HubClient::new(HOST_NAME, DEVICE_ID, Option::None).unwrap();
        let mut out: String = String::with_capacity(200);
        let rc = client.ll_get_client_id(&mut out);
        assert_eq!(rc, AzReturnCode::AzResultCoreOk);
        assert_eq!(DEVICE_ID, out);
    }
    #[test]
    fn client_ll_get_client_id_fail() {
        let client = HubClient::new(HOST_NAME, DEVICE_ID, Option::None).unwrap();
        let mut out: String = String::with_capacity(2);
        let rc = client.ll_get_client_id(&mut out);
        assert_eq!(rc, AzReturnCode::AzResultCoreErrorNotEnoughSpace);
    }
    #[test]
    fn client_get_user_name() {
        let client = HubClient::new(HOST_NAME, DEVICE_ID, Option::None).unwrap();
        let user_name = HOST_NAME.to_string()
            + "/"
            + DEVICE_ID
            + "/?api-version=2018-06-30&DeviceClientType=c%2F1.0.0";
        assert_eq!(user_name, client.get_user_name().unwrap());
    }
    #[test]
    fn client_ll_get_user_name() {
        let client = HubClient::new(HOST_NAME, DEVICE_ID, Option::None).unwrap();
        let user_name = HOST_NAME.to_string()
            + "/"
            + DEVICE_ID
            + "/?api-version=2018-06-30&DeviceClientType=c%2F1.0.0";
        let mut out: String = String::with_capacity(200);
        let rc = client.ll_get_user_name(&mut out);
        assert_eq!(rc, AzReturnCode::AzResultCoreOk);
        assert_eq!(user_name, out);
    }
    #[test]
    fn client_get_telemetry_publish_topic() {
        let client = HubClient::new(HOST_NAME, DEVICE_ID, Option::None).unwrap();
        let topic = "devices/".to_string() + DEVICE_ID + "/messages/events/";
        assert_eq!(
            topic,
            client.get_telemetry_publish_topic(Option::None).unwrap()
        );
    }
    #[test]
    fn client_ll_get_telemetry_publish_topic() {
        let client = HubClient::new(HOST_NAME, DEVICE_ID, Option::None).unwrap();
        let topic = "devices/".to_string() + DEVICE_ID + "/messages/events/";
        let mut out = String::with_capacity(200);
        let rc = client.ll_get_telemetry_publish_topic(&Option::None, &mut out);
        assert_eq!(rc, AzReturnCode::AzResultCoreOk);
        assert_eq!(topic, out);
    }
    #[test]
    fn client_get_sas_signature() {
        let client = HubClient::new(HOST_NAME, DEVICE_ID, Option::None).unwrap();
        let signature = HOST_NAME.to_string() + "%2Fdevices%2F" + DEVICE_ID + "\n100"; // &DeviceClientType=c%2F1.0.0";
        assert_eq!(
            String::from_utf8_lossy(&client.get_sas_signature(100).unwrap()),
            signature
        );
    }
    #[test]
    fn client_ll_get_sas_signature() {
        let client = HubClient::new(HOST_NAME, DEVICE_ID, Option::None).unwrap();
        let signature = HOST_NAME.to_string() + "%2Fdevices%2F" + DEVICE_ID + "\n100"; // &DeviceClientType=c%2F1.0.0";
        let mut out: Vec<u8> = Vec::with_capacity(200);
        let rc = client.ll_get_sas_signature(100, &mut out);
        assert_eq!(rc, AzReturnCode::AzResultCoreOk);
        assert_eq!(String::from_utf8_lossy(&out), signature);
    }
    #[test]
    fn client_get_sas_password() {
        let client = HubClient::new(HOST_NAME, DEVICE_ID, Option::None).unwrap();
        let sas = "NotReallyASASToken";
        let password = "SharedAccessSignature sr=".to_string()
            + HOST_NAME
            + "%2Fdevices%2F"
            + DEVICE_ID
            + "&sig="
            + sas
            + "&se=100";
        assert_eq!(&password, &client.get_sas_password(100, sas).unwrap());
    }
    #[test]
    fn client_ll_get_sas_password() {
        let client = HubClient::new(HOST_NAME, DEVICE_ID, Option::None).unwrap();
        let sas = "NotReallyASASToken";
        let password = "SharedAccessSignature sr=".to_string()
            + HOST_NAME
            + "%2Fdevices%2F"
            + DEVICE_ID
            + "&sig="
            + sas
            + "&se=100";
        let mut out = String::with_capacity(200);
        let rc = client.ll_get_sas_password(100, sas, &mut out);
        assert_eq!(rc, AzReturnCode::AzResultCoreOk);
        assert_eq!(out, password);
    }
    #[test]
    fn client_get_c2d_subscribe_topic() {
        assert_eq!(
            HubClient::get_c2d_subscribe_topic(),
            "devices/+/messages/devicebound/#"
        );
    }
    #[test]
    fn client_get_method_subscribe_topic() {
        assert_eq!(
            HubClient::get_method_subscribe_topic(),
            "$iothub/methods/POST/#"
        );
    }
}
