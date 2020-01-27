-- drop indexes
drop index idx_unique_wallpaper_sizes;
drop index idx_unique_photo_wallpaper_combo;

-- Drop `wallpaper_display_names` view
drop view if exists wallpaper_display_names;

-- Drop `photo_wallpaper` junction table
drop table if exists photo_wallpaper;

-- Drop `wallpaper_sizes` table
drop table if exists wallpaper_sizes;