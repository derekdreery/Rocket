macro_rules! known_media_types {
    ($cont:ident) => {
        $cont! {
            Any (is_any): "any media type", "*", "*",
            Binary (is_binary): "binary data", "application", "octet-stream",
            HTML (is_html): "HTML", "text", "html" ; "charset" => "utf-8",
            Plain (is_plain): "plain text", "text", "plain" ; "charset" => "utf-8",
            JSON (is_json): "JSON", "application", "json",
            MsgPack (is_msgpack): "MsgPack", "application", "msgpack",
            Form (is_form): "forms", "application", "x-www-form-urlencoded",
            JavaScript (is_javascript): "JavaScript", "application", "javascript",
            CSS (is_css): "CSS", "text", "css" ; "charset" => "utf-8",
            FormData (is_form_data): "multipart form data", "multipart", "form-data",
            XML (is_xml): "XML", "text", "xml" ; "charset" => "utf-8",
            CSV (is_csv): "CSV", "text", "csv" ; "charset" => "utf-8",
            PNG (is_png): "PNG", "image", "png",
            GIF (is_gif): "GIF", "image", "gif",
            BMP (is_bmp): "BMP", "image", "bmp",
            JPEG (is_jpeg): "JPEG", "image", "jpeg",
            WEBP (is_webp): "WEBP", "image", "webp",
            SVG (is_svg): "SVG", "image", "svg+xml",
            Icon (is_icon): "Icon", "image", "x-icon",
            WEBM (is_webm): "WEBM", "video", "webm",
            WEBA (is_weba): "WEBM Audio", "audio", "webm",
            OGG (is_ogg): "OGG Video", "video", "ogg",
            FLAC (is_flac): "FLAC", "audio", "flac",
            WAV (is_wav): "WAV", "audio", "wav",
            PDF (is_pdf): "PDF", "application", "pdf",
            TTF (is_ttf): "TTF", "application", "font-sfnt",
            OTF (is_otf): "OTF", "application", "font-sfnt",
            WOFF (is_woff): "WOFF", "application", "font-woff",
            WOFF2 (is_woff2): "WOFF2", "font", "woff2",
            JsonApi (is_json_api): "JSON API", "application", "vnd.api+json",
            WASM (is_wasm): "WASM", "application", "wasm",
            TIFF (is_tiff): "TIFF", "image", "tiff",
            AAC (is_aac): "AAC Audio", "audio", "aac",
            Calendar (is_ical): "iCalendar", "text", "calendar",
            MPEG (is_mpeg): "MPEG Video", "video", "mpeg",
            TAR (is_tar): "tape archive", "application", "x-tar",
            GZIP (is_gzip): "gzipped binary", "application", "gzip",
            MOV (is_mov): "quicktime video", "video", "quicktime",
            MP4 (is_mp4): "MPEG4 Video", "video", "mp4",
            ZIP (is_zip): "ZIP archive", "application", "zip",
        }
    };
}

macro_rules! known_extensions {
    ($cont:ident) => {
        $cont! {
            "txt" => Plain,
            "html" => HTML,
            "htm" => HTML,
            "xml" => XML,
            "csv" => CSV,
            "js" => JavaScript,
            "css" => CSS,
            "json" => JSON,
            "png" => PNG,
            "gif" => GIF,
            "bmp" => BMP,
            "jpeg" => JPEG,
            "jpg" => JPEG,
            "webp" => WEBP,
            "svg" => SVG,
            "ico" => Icon,
            "flac" => FLAC,
            "wav" => WAV,
            "webm" => WEBM,
            "weba" => WEBA,
            "ogg" => OGG,
            "ogv" => OGG,
            "pdf" => PDF,
            "ttf" => TTF,
            "otf" => OTF,
            "woff" => WOFF,
            "woff2" => WOFF2,
            "mp4" => MP4,
            "mpeg4" => MP4,
            "wasm" => WASM,
            "aac" => AAC,
            "ics" => Calendar,
            "bin" => Binary,
            "mpg" => MPEG,
            "mpeg" => MPEG,
            "tar" => TAR,
            "gz" => GZIP,
            "tif" => TIFF,
            "tiff" => TIFF,
            "mov" => MOV,
            "zip" => ZIP,
        }
    };
}

macro_rules! known_shorthands {
    ($cont:ident) => {
        $cont! {
            "any" => Any,
            "binary" => Binary,
            "html" => HTML,
            "plain" => Plain,
            "json" => JSON,
            "msgpack" => MsgPack,
            "form" => Form,
            "js" => JavaScript,
            "css" => CSS,
            "multipart" => FormData,
            "xml" => XML,
        }
    };
}
