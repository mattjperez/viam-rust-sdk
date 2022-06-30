// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPosesRequest {
    /// Name of the pose tracker
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Names of the bodies whose poses are being requested. In the event
    /// this parameter is not supplied or is an empty list, all available
    /// poses are returned
    #[prost(string, repeated, tag="2")]
    pub body_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPosesResponse {
    /// Mapping of each body name to the pose representing the center of the body.
    #[prost(map="string, message", tag="1")]
    pub body_poses: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::common::v1::PoseInFrame>,
}
/// Encoded file descriptor set for the `proto.api.component.posetracker.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xb0, 0x0c, 0x0a, 0x35, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x63,
    0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x70, 0x6f, 0x73, 0x65, 0x74, 0x72, 0x61,
    0x63, 0x6b, 0x65, 0x72, 0x2f, 0x76, 0x31, 0x2f, 0x70, 0x6f, 0x73, 0x65, 0x5f, 0x74, 0x72, 0x61,
    0x63, 0x6b, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x22, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e,
    0x70, 0x6f, 0x73, 0x65, 0x74, 0x72, 0x61, 0x63, 0x6b, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x1a, 0x1c,
    0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x20, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x76,
    0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x44,
    0x0a, 0x0f, 0x47, 0x65, 0x74, 0x50, 0x6f, 0x73, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x62, 0x6f, 0x64, 0x79, 0x5f, 0x6e, 0x61,
    0x6d, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x52, 0x09, 0x62, 0x6f, 0x64, 0x79, 0x4e,
    0x61, 0x6d, 0x65, 0x73, 0x22, 0xd6, 0x01, 0x0a, 0x10, 0x47, 0x65, 0x74, 0x50, 0x6f, 0x73, 0x65,
    0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x62, 0x0a, 0x0a, 0x62, 0x6f, 0x64,
    0x79, 0x5f, 0x70, 0x6f, 0x73, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x43, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e,
    0x65, 0x6e, 0x74, 0x2e, 0x70, 0x6f, 0x73, 0x65, 0x74, 0x72, 0x61, 0x63, 0x6b, 0x65, 0x72, 0x2e,
    0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x50, 0x6f, 0x73, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x2e, 0x42, 0x6f, 0x64, 0x79, 0x50, 0x6f, 0x73, 0x65, 0x73, 0x45, 0x6e, 0x74,
    0x72, 0x79, 0x52, 0x09, 0x62, 0x6f, 0x64, 0x79, 0x50, 0x6f, 0x73, 0x65, 0x73, 0x1a, 0x5e, 0x0a,
    0x0e, 0x42, 0x6f, 0x64, 0x79, 0x50, 0x6f, 0x73, 0x65, 0x73, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12,
    0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6b, 0x65,
    0x79, 0x12, 0x36, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x20, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x63, 0x6f, 0x6d,
    0x6d, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x50, 0x6f, 0x73, 0x65, 0x49, 0x6e, 0x46, 0x72, 0x61,
    0x6d, 0x65, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x3a, 0x02, 0x38, 0x01, 0x32, 0xc6, 0x01,
    0x0a, 0x12, 0x50, 0x6f, 0x73, 0x65, 0x54, 0x72, 0x61, 0x63, 0x6b, 0x65, 0x72, 0x53, 0x65, 0x72,
    0x76, 0x69, 0x63, 0x65, 0x12, 0xaf, 0x01, 0x0a, 0x08, 0x47, 0x65, 0x74, 0x50, 0x6f, 0x73, 0x65,
    0x73, 0x12, 0x33, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x63, 0x6f,
    0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x70, 0x6f, 0x73, 0x65, 0x74, 0x72, 0x61, 0x63,
    0x6b, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x50, 0x6f, 0x73, 0x65, 0x73, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x34, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x61,
    0x70, 0x69, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x70, 0x6f, 0x73,
    0x65, 0x74, 0x72, 0x61, 0x63, 0x6b, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x50,
    0x6f, 0x73, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x38, 0x82, 0xd3,
    0xe4, 0x93, 0x02, 0x32, 0x12, 0x30, 0x2f, 0x76, 0x69, 0x61, 0x6d, 0x2f, 0x61, 0x70, 0x69, 0x2f,
    0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x70, 0x6f, 0x73,
    0x65, 0x5f, 0x74, 0x72, 0x61, 0x63, 0x6b, 0x65, 0x72, 0x2f, 0x7b, 0x6e, 0x61, 0x6d, 0x65, 0x7d,
    0x2f, 0x70, 0x6f, 0x73, 0x65, 0x73, 0x42, 0x4d, 0x0a, 0x23, 0x63, 0x6f, 0x6d, 0x2e, 0x76, 0x69,
    0x61, 0x6d, 0x2e, 0x72, 0x64, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x61, 0x70, 0x69,
    0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x76, 0x31, 0x5a, 0x26, 0x67,
    0x6f, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x72, 0x64, 0x6b, 0x2f, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65,
    0x6e, 0x74, 0x2f, 0x76, 0x31, 0x4a, 0xd3, 0x06, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x20, 0x01,
    0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12,
    0x03, 0x02, 0x00, 0x2b, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x26, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x2a, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x07, 0x00, 0x3d, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x07, 0x00, 0x3d, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x3c, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12,
    0x03, 0x08, 0x00, 0x3c, 0x0a, 0x56, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x0b, 0x00, 0x12, 0x01,
    0x1a, 0x4a, 0x20, 0x41, 0x20, 0x50, 0x6f, 0x73, 0x65, 0x54, 0x72, 0x61, 0x63, 0x6b, 0x65, 0x72,
    0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x6d, 0x61, 0x69, 0x6e, 0x74, 0x61, 0x69, 0x6e,
    0x73, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x70, 0x6f, 0x73, 0x65, 0x20, 0x74, 0x72, 0x61, 0x63, 0x6b,
    0x65, 0x72, 0x73, 0x20, 0x61, 0x73, 0x73, 0x6f, 0x63, 0x69, 0x61, 0x74, 0x65, 0x64, 0x20, 0x77,
    0x69, 0x74, 0x68, 0x20, 0x61, 0x20, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03,
    0x06, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x1a, 0x0a, 0x5a, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00,
    0x12, 0x04, 0x0d, 0x02, 0x11, 0x03, 0x1a, 0x4c, 0x20, 0x47, 0x65, 0x74, 0x50, 0x6f, 0x73, 0x65,
    0x73, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75,
    0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x70, 0x6f, 0x73, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x65, 0x61,
    0x63, 0x68, 0x20, 0x62, 0x6f, 0x64, 0x79, 0x20, 0x74, 0x72, 0x61, 0x63, 0x6b, 0x65, 0x64, 0x20,
    0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x6f, 0x73, 0x65, 0x20, 0x74, 0x72, 0x61, 0x63,
    0x6b, 0x65, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d,
    0x06, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0d, 0x0f, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x29, 0x39, 0x0a, 0x0d,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0e, 0x04, 0x10, 0x06, 0x0a, 0x11, 0x0a,
    0x09, 0x06, 0x00, 0x02, 0x00, 0x04, 0xb0, 0xca, 0xbc, 0x22, 0x12, 0x04, 0x0e, 0x04, 0x10, 0x06,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x14, 0x00, 0x1b, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x14, 0x08, 0x17, 0x0a, 0x27, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x16, 0x02, 0x12, 0x1a, 0x1a, 0x20, 0x4e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x70, 0x6f, 0x73, 0x65, 0x20, 0x74, 0x72, 0x61, 0x63, 0x6b, 0x65, 0x72,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x16, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x09, 0x0d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x16, 0x10, 0x11, 0x0a, 0xa8, 0x01, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x02, 0x21, 0x1a, 0x9a, 0x01, 0x20, 0x4e, 0x61, 0x6d,
    0x65, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x6f, 0x64, 0x69, 0x65, 0x73,
    0x20, 0x77, 0x68, 0x6f, 0x73, 0x65, 0x20, 0x70, 0x6f, 0x73, 0x65, 0x73, 0x20, 0x61, 0x72, 0x65,
    0x20, 0x62, 0x65, 0x69, 0x6e, 0x67, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64,
    0x2e, 0x20, 0x49, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x0a, 0x20,
    0x74, 0x68, 0x69, 0x73, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x20, 0x69,
    0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x20, 0x6f,
    0x72, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x20, 0x6c, 0x69,
    0x73, 0x74, 0x2c, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c,
    0x65, 0x0a, 0x20, 0x70, 0x6f, 0x73, 0x65, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x72, 0x65, 0x74,
    0x75, 0x72, 0x6e, 0x65, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x1a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1a,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1a, 0x12, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1a, 0x1f, 0x20, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x1d, 0x00, 0x20, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01,
    0x01, 0x12, 0x03, 0x1d, 0x08, 0x18, 0x0a, 0x59, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x1f, 0x02, 0x34, 0x1a, 0x4c, 0x20, 0x4d, 0x61, 0x70, 0x70, 0x69, 0x6e, 0x67, 0x20, 0x6f, 0x66,
    0x20, 0x65, 0x61, 0x63, 0x68, 0x20, 0x62, 0x6f, 0x64, 0x79, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20,
    0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x6f, 0x73, 0x65, 0x20, 0x72, 0x65, 0x70, 0x72,
    0x65, 0x73, 0x65, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x65, 0x6e,
    0x74, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x6f, 0x64, 0x79, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1f, 0x02, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1f, 0x25, 0x2f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1f, 0x32, 0x33, 0x62, 0x06, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x33,
];
include!("proto.api.component.posetracker.v1.tonic.rs");
// @@protoc_insertion_point(module)