# utility script that writes a csv file in the same format as the database for
# songs and charts
# you must provide your own songlist and packlist somehow, good luck :)

import json
import csv
from datetime import datetime

with open("songlist", encoding = "utf-8") as songlist_file:
    songlist = json.load(songlist_file)

with open("packlist", encoding="utf-8") as packlist_file:
    packlist = json.load(packlist_file)

with open("songs.csv", "w", newline="", encoding="utf-8") as song_file:
    writer = csv.writer(song_file)

    writer.writerow(["ingame_id", "title", "artist", "release_date", "pack"])
    for song in songlist["songs"]:
        for pack in packlist["packs"]:
            if song["set"] == pack["id"]:
                if "append" in pack["id"]:
                    for parent_pack in packlist["packs"]:
                        if pack["pack_parent"] == parent_pack["id"]:
                            if pack["pack_parent"] in ["chunithm", "ongeki", "cytusii", "wacca", "maimai", "lanota"]:
                                pack_name = parent_pack["name_localized"]["en"] + " " + pack["name_localized"]["en"]
                            elif pack["id"] == "eden_append_1":
                                pack_name = pack["name_localized"]["en"]
                            else:
                                pack_name = parent_pack["name_localized"]["en"] + " - " +  pack["name_localized"]["en"]
                else:
                    pack_name = pack["name_localized"]["en"]
                    if pack["id"] in ["chunithm", "ongeki", "cytusii", "wacca", "maimai", "lanota"]:
                        pack_name += " Collaboration"

        writer.writerow([song["id"], song["title_localized"]["en"], song["artist"], datetime.fromtimestamp(song["date"]).isoformat() + "+0700", pack_name])

with open("charts.csv", "w", newline="", encoding="utf-8") as chart_file:
    writer = csv.writer(chart_file)

    writer.writerow(["song_id", "difficulty", "level", "constant", "charter"])
    # for song in songlist["songs"]: