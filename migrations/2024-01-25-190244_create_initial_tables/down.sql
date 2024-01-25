-- Drop foreign key constraints in reverse order
ALTER TABLE post_tags DROP CONSTRAINT IF EXISTS post_tags_post_id_fkey;
ALTER TABLE post_tags DROP CONSTRAINT IF EXISTS post_tags_tag_id_fkey;
ALTER TABLE posts DROP CONSTRAINT IF EXISTS fk_category_post;
ALTER TABLE posts DROP CONSTRAINT IF EXISTS fk_user_post;
ALTER TABLE profiles DROP CONSTRAINT IF EXISTS fk_user_profile;

-- Drop tables in reverse order
DROP TABLE IF EXISTS post_tags;
DROP TABLE IF EXISTS posts;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS categories;
DROP TABLE IF EXISTS profiles;
DROP TABLE IF EXISTS users;
