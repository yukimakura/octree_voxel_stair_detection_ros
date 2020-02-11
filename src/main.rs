use std::sync::{Mutex, Arc};

mod v_msg {
    rosrust::rosmsg_include!(visualization_msgs/Marker);
}

mod f_msg {
    rosrust::rosmsg_include!(std_msgs/Float64);
}

fn main() {
    // Initialize node
    rosrust::init("Marker_listener");

    let mut x0_pub_mutex = Arc::new(Mutex::new(rosrust::publish("/x0points", 100).unwrap()));
    let marker_sub = rosrust::subscribe("/jsk/marker", 100, move |v: v_msg::visualization_msgs::Marker| {

        let mut x0_pub = x0_pub_mutex.lock().unwrap();
        // Callback for handling received messages
        let mut delta_z = f_msg::std_msgs::Float64::default();
        let mut x0_points : Vec<v_msg::geometry_msgs::Point> = Vec::new();
        let mut x0_pub_msgs = v.clone();
        let mut x0_colors : Vec<v_msg::std_msgs::ColorRGBA> = Vec::new();

        // rosrust::ros_info!("Received: {}, Color :{:?}", &v.points.len(),&v.colors.get(5));
        let mut min_y = v_msg::geometry_msgs::Point{x:0.0,y:0.0,z:100.0};
        let mut delta_y_point = v_msg::geometry_msgs::Point::default();
        for (cnt,item) in v.points.iter().enumerate(){
            if item.x.abs() <= 0.05 {
                let red = v_msg::std_msgs::ColorRGBA{r:1.0,g:0.0,b:0.0,a:1.0,};
                x0_points.push(v_msg::geometry_msgs::Point::clone(item));
                // x0_colors.push(v_msg::std_msgs::ColorRGBA::clone(&v.colors[cnt]));
                x0_colors.push(red);
                if min_y.y > item.y{
                    min_y = v_msg::geometry_msgs::Point::clone(item);
                }
            }
        }
        for item in &x0_points{
            if (min_y.y + 0.2) < item.y  && item.y < (min_y.y + 0.3){ //一番下のポイントから大体20〜30cmぐらい上のポイントを捜索
                delta_y_point = v_msg::geometry_msgs::Point::clone(item);
            }
        }
        rosrust::ros_info!("min_y point: {:?},\tdelta_y_point: {:?},\t z delta: {}",min_y.y,delta_y_point.y,min_y.z-delta_y_point.z);
        rosrust::ros_info!("z info = (min_y point: {:?},\tdelta_y_point: {:?})",min_y.z,delta_y_point.z);
        x0_pub_msgs.type_ = 6;
        x0_pub_msgs.points = x0_points;
        x0_pub_msgs.colors = x0_colors;
        x0_pub_msgs.scale.x = 3.0;
        x0_pub_msgs.scale.y = 3.0;

        x0_pub.send(x0_pub_msgs).unwrap();
    }).unwrap();

    rosrust::spin();
}