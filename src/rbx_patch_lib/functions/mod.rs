use super::{
    Result,
    errors::{ CouldntFindBytePattern, CouldntConvertToi64 },
    constants::enum_bytes::{
        CLIENT
    },
    enums::BinaryType,
};
use super::enums::BinaryType::*;

pub fn find_instances_of_bytes(source: &[u8], pattern: &[u8]) -> Vec<usize> {
    let mut found_instances: Vec<usize> = vec![];

    for (index, byte) in source.iter().enumerate() {
        if byte != &pattern[0] {
            continue;
        }
        let mut failed = false;
        for i in 0..pattern.len() {
            if index + i > source.len() {
                failed = true;
                break;
            }
            let current_byte = source[index + i];
            let check_byte = pattern[i];

            if current_byte != check_byte {
                failed = true;
                break;
            }
        }

        if failed == true {
            continue;
        }

        found_instances.push(index);
    }

    return found_instances;
}

pub fn replace_first_instance_of_bytes(
    source: &mut [u8],
    pattern: &[u8],
    replacement: &[u8]
) -> Result<()> {
    let replacement_index = find_instances_of_bytes(&source, pattern);

    if replacement_index.len() == 0 {
        return Err(CouldntFindBytePattern.into());
    }

    let index: Result<&usize> = replacement_index.first().ok_or(CouldntFindBytePattern.into());
    let index = index?;

    for (index1, byte) in replacement.iter().enumerate() {
        source[index + index1] = byte.clone();
    }

    Ok(())
}
pub fn replace_last_instance_of_bytes(
    source: &mut [u8],
    pattern: &[u8],
    replacement: &[u8]
) -> Result<()> {
    let replacement_index = find_instances_of_bytes(&source, pattern);

    if replacement_index.len() == 0 {
        return Err(CouldntFindBytePattern.into());
    }

    let index: Result<&usize> = replacement_index.last().ok_or(CouldntFindBytePattern.into());
    let index = index?;

    for (index1, byte) in replacement.iter().enumerate() {
        source[index + index1] = byte.clone();
    }

    Ok(())
}
pub fn replace_all_instances_of_bytes(
    source: &mut [u8],
    pattern: &[u8],
    replacement: &[u8]
) -> Result<()> {
    let replacement_index = find_instances_of_bytes(&source, pattern);

    if replacement_index.len() == 0 {
        return Err(CouldntFindBytePattern.into());
    }

    for index in replacement_index {
        for (index1, byte) in replacement.iter().enumerate() {
            source[index + index1] = byte.clone();
        }
    }

    Ok(())
}

pub fn replace_closest_to_index(
    source: &mut [u8],
    index: &usize,
    pattern: &[u8],
    replacement: &[u8]
) -> Result<()> {
    let replace_ments = find_instances_of_bytes(&source, pattern);

    let mut closest = None;
    let converted = usize_to_i64(index.to_owned())?;

    for index_s in replace_ments {
        let mut distance = usize_to_i64(index_s)? - converted;

        if distance.is_negative() {
            distance = -1 * distance;
        }
        if closest.is_none() {
            closest = Some((index_s, distance));
            continue;
        }

        let (_, distance_targ) = closest.unwrap();
        if distance_targ > distance {
            closest = Some((index_s, distance));
        }
    }

    if closest.is_none() {
        return Err(CouldntFindBytePattern.into());
    }

    let (index, _) = closest.unwrap();
    for (index1, byte) in replacement.iter().enumerate() {
        source[index + index1] = byte.clone();
    }

    Ok(())
}

#[cfg(target_pointer_width = "64")]
fn usize_to_i64(isize: usize) -> Result<i64> {
    if isize > (i64::MAX as usize) {
        return Err(CouldntConvertToi64.into());
    }

    return Ok(isize as i64);
}

pub fn client_types_to_bytes(types: Vec<super::enums::BinaryType>) -> u8 {
    

    if types.len() == 1 {
        return match types[0] {
            Client => CLIENT,
        };
    }

    /* 
    let first = types[0];
    let last = types[1];

    if types.contains(&Client) && types.contains(&Studio) {
        return CLIENT_STUDIO;
    }
    if types.contains(&Client) && types.contains(&RCCService) {
        return CLIENT_RCC;
    }
    if types.contains(&Studio) && types.contains(&RCCService) {
        return RCC_STUDIO;
    }
    */
    return 0x0;
}

pub fn bytes_to_client_type(bin: u8) -> Vec<BinaryType> {
    println!("{bin}");
    match bin {
        CLIENT => vec![Client],
        _ => vec![],
    }
}
