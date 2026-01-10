mod utils;

use crate::utils::{login, test_get_ok};
use serde_json::json;

const DEV_BASE_URL: &str = "http://localhost:1948/api/v1";

test_get_ok!(
    test_name = get_pitches_ok_no_login,
    path = "/pitches",
    status = 200,
    response = json!([
        {
            "id": "00000000-0000-0000-0000-000000000006",
            "owner_id": "00000000-0000-0000-0000-000000000003",
            "display_name": "football_pitch_1",
            "sport": "football"
        },
        {
            "id": "00000000-0000-0000-0000-000000000007",
            "owner_id": "00000000-0000-0000-0000-000000000003",
            "display_name": "football_pitch_2",
            "sport": "football"
        },
        {
            "id": "00000000-0000-0000-0000-000000000008",
            "owner_id": "00000000-0000-0000-0000-000000000004",
            "display_name": "basketball_pitch_1",
            "sport": "basketball"
        },
        {
            "id": "00000000-0000-0000-0000-000000000009",
            "owner_id": "00000000-0000-0000-0000-000000000004",
            "display_name": "padel_pitch_1",
            "sport": "padel"
        }
    ])
);

test_get_ok!(
    test_name = get_pitches_ok_player,
    user = player_2,
    path = "/pitches",
    status = 200,
    response = json!([
        {
            "id": "00000000-0000-0000-0000-000000000006",
            "owner_id": "00000000-0000-0000-0000-000000000003",
            "display_name": "football_pitch_1",
            "sport": "football"
        },
        {
            "id": "00000000-0000-0000-0000-000000000007",
            "owner_id": "00000000-0000-0000-0000-000000000003",
            "display_name": "football_pitch_2",
            "sport": "football"
        },
        {
            "id": "00000000-0000-0000-0000-000000000008",
            "owner_id": "00000000-0000-0000-0000-000000000004",
            "display_name": "basketball_pitch_1",
            "sport": "basketball"
        },
        {
            "id": "00000000-0000-0000-0000-000000000009",
            "owner_id": "00000000-0000-0000-0000-000000000004",
            "display_name": "padel_pitch_1",
            "sport": "padel"
        }
    ])
);

test_get_ok!(
    test_name = get_pitches_ok_business,
    user = business_2,
    path = "/pitches",
    status = 200,
    response = json!([
        {
            "id": "00000000-0000-0000-0000-000000000008",
            "owner_id": "00000000-0000-0000-0000-000000000004",
            "display_name": "basketball_pitch_1",
            "sport": "basketball"
        },
        {
            "id": "00000000-0000-0000-0000-000000000009",
            "owner_id": "00000000-0000-0000-0000-000000000004",
            "display_name": "padel_pitch_1",
            "sport": "padel"
        }
    ])
);

test_get_ok!(
    test_name = get_pitches_ok_admin,
    user = admin,
    path = "/pitches",
    status = 200,
    response = json!([
            {
                "id": "00000000-0000-0000-0000-000000000006",
                "owner_id": "00000000-0000-0000-0000-000000000003",
                "display_name": "football_pitch_1",
                "sport": "football"
            },
            {
                "id": "00000000-0000-0000-0000-000000000007",
                "owner_id": "00000000-0000-0000-0000-000000000003",
                "display_name": "football_pitch_2",
                "sport": "football"
            },
            {
                "id": "00000000-0000-0000-0000-000000000008",
                "owner_id": "00000000-0000-0000-0000-000000000004",
                "display_name": "basketball_pitch_1",
                "sport": "basketball"
            },
            {
                "id": "00000000-0000-0000-0000-000000000009",
                "owner_id": "00000000-0000-0000-0000-000000000004",
                "display_name": "padel_pitch_1",
                "sport": "padel"
            }
    ])
);
