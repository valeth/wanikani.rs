# Filters

## `GET` `/subjects`
Name                | Type
------------------- | --------
`ids`               | String
`types`             | String
`slugs`             | String
`levels`            | String
`updated_after`     | String

## `GET` `/assignments`
Name               | Type
------------------ | --------
`ids`              | String
`created_at`       | Date
`subject_ids`      | String
`subject_types`    | String
`levels`           | String
`available_before` | String
`available_after`  | String
`srs_stages`       | String
`unlocked`         | Boolean
`started`          | Boolean
`passed`           | Boolean
`burned`           | Boolean
`resurrected`      | Boolean
`updated_after`    | String

## `GET` `/review_statistics`
Name                       | Type
-------------------------- | --------
`ids`                      | Integer
`subject_ids`              | Integer
`subject_types`            | String
`updated_after`            | String
`percentages_greater_than` | Integer
`percentages_less_than`    | Integer

## `GET` `/study_materials`
Name                       | Type
-------------------------- | --------
`ids`                      | String
`subject_ids`              | String
`subject_types`            | String
`updated_after`            | String

## `GET` `/reviews`
Name                       | Type
-------------------------- | --------
`ids`                      | String
`assignment_ids`           | String
`subject_ids`              | String
`updated_after`            | String

## `GET` `/level_progressions`
Name                       | Type
-------------------------- | --------
`ids`                      | String
`updated_after`            | String

## `GET` `/resets`
Name                       | Type
-------------------------- | --------
`ids`                      | String
`updated_after`            | String
