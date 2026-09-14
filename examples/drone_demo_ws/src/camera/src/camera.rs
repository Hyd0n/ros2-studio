fn main() -> anyhow::Result<()> {
    let context = rclrs::Context::default();
    let mut node = rclrs::create_node(&context, "camera")?;

    let _publisher = node.create_publisher::<sensor_msgs::msg::Image>(
        "/camera/image",
        rclrs::QOS_PROFILE_DEFAULT,
    )?;

    rclrs::spin(&node)?;
    Ok(())
}
