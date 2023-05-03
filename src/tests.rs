use super::*;

#[test]
fn test() {
    let edges = read_files("hero-network.csv");
    let first_edge: (String, String) = ("LITTLE, ABNER".to_string(), "PRINCESS ZANDA".to_string());
    let second_edge = ("LITTLE, ABNER".to_string(), "BLACK PANTHER/T'CHAL".to_string()); 
    let third_edge = ("BLACK PANTHER/T'CHAL".to_string(), "PRINCESS ZANDA".to_string());
    assert_eq!(edges[0], first_edge);
    assert_eq!(edges[1], second_edge);
    assert_eq!(edges[2], third_edge);
    
}