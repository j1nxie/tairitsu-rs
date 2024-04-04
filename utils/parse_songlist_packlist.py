# utility script that writes a csv file in the same format as the database for
# songs and charts
# you must provide your own songlist and packlist somehow, good luck :)

from decimal import Decimal
import json
import csv
from datetime import datetime

with open("songlist", encoding="utf-8") as songlist_file:
    songlist = json.load(songlist_file)

with open("packlist", encoding="utf-8") as packlist_file:
    packlist = json.load(packlist_file)

with open("songs.csv", "w", newline="", encoding="utf-8") as song_file:
    writer = csv.writer(song_file)

    writer.writerow(
        ["ingame_id", "title", "artist", "release_date", "version", "bpm", "pack"]
    )
    for song in songlist["songs"]:
        for pack in packlist["packs"]:
            if song["set"] == pack["id"]:
                if "append" in pack["id"]:
                    for parent_pack in packlist["packs"]:
                        if pack["pack_parent"] == parent_pack["id"]:
                            if pack["pack_parent"] in [
                                "chunithm",
                                "ongeki",
                                "cytusii",
                                "wacca",
                                "maimai",
                                "lanota",
                            ]:
                                pack_name = (
                                    parent_pack["name_localized"]["en"]
                                    + " "
                                    + pack["name_localized"]["en"]
                                )
                            elif pack["id"] == "eden_append_1":
                                pack_name = pack["name_localized"]["en"]
                            else:
                                pack_name = (
                                    parent_pack["name_localized"]["en"]
                                    + " - "
                                    + pack["name_localized"]["en"]
                                )
                else:
                    pack_name = pack["name_localized"]["en"]
                    if pack["id"] in [
                        "chunithm",
                        "ongeki",
                        "cytusii",
                        "wacca",
                        "maimai",
                        "lanota",
                    ]:
                        pack_name += " Collaboration"
            elif song["set"] == "single":
                pack_name = "Memory Archive"

        writer.writerow(
            [
                song["id"],
                song["title_localized"]["en"],
                song["artist"],
                datetime.fromtimestamp(song["date"]).isoformat() + "+0700",
                song["version"],
                song["bpm"],
                pack_name,
            ]
        )

with open("charts.csv", "w", newline="", encoding="utf-8") as chart_file:
    with open("songs.csv", "a", newline="", encoding="utf-8") as song_file:
        writer = csv.writer(chart_file)
        song_writer = csv.writer(song_file)

        writer.writerow(["song_id", "difficulty", "level", "constant", "charter"])
        for song in songlist["songs"]:
            for chart in song["difficulties"]:
                diff_dict = {0: "PST", 1: "PRS", 2: "FTR", 3: "BYD", 4: "ETR"}
                song_id = song["id"]

                if chart["ratingClass"] == 3:
                    if "title_localized" in chart.keys():
                        song_id += "_byd"
                        for pack in packlist["packs"]:
                            if song["set"] == pack["id"]:
                                if "append" in pack["id"]:
                                    for parent_pack in packlist["packs"]:
                                        if pack["pack_parent"] == parent_pack["id"]:
                                            if pack["pack_parent"] in [
                                                "chunithm",
                                                "ongeki",
                                                "cytusii",
                                                "wacca",
                                                "maimai",
                                                "lanota",
                                            ]:
                                                pack_name = (
                                                    parent_pack["name_localized"]["en"]
                                                    + " "
                                                    + pack["name_localized"]["en"]
                                                )
                                            elif pack["id"] == "eden_append_1":
                                                pack_name = pack["name_localized"]["en"]
                                            else:
                                                pack_name = (
                                                    parent_pack["name_localized"]["en"]
                                                    + " - "
                                                    + pack["name_localized"]["en"]
                                                )
                                else:
                                    pack_name = pack["name_localized"]["en"]
                                    if pack["id"] in [
                                        "chunithm",
                                        "ongeki",
                                        "cytusii",
                                        "wacca",
                                        "maimai",
                                        "lanota",
                                    ]:
                                        pack_name += " Collaboration"
                            elif song["set"] == "single":
                                pack_name = "Memory Archive"

                        song_writer.writerow(
                            [
                                song_id,
                                chart["title_localized"]["en"],
                                chart["artist"]
                                if "artist" in chart.keys()
                                else song["artist"],
                                datetime.fromtimestamp(
                                    chart["date"]
                                    if "date" in chart.keys()
                                    else song["date"]
                                ).isoformat()
                                + "+0700",
                                chart["version"]
                                if "version" in chart.keys()
                                else song["version"],
                                chart["bpm"] if "bpm" in chart.keys() else song["bpm"],
                                pack_name,
                            ]
                        )

                chart_level = str(chart["rating"])
                if "ratingPlus" in chart.keys() and chart["ratingPlus"]:
                    chart_level += "+"

                chart_constant = Decimal(chart["rating"])
                if "ratingPlus" in chart.keys() and chart["ratingPlus"]:
                    chart_constant += Decimal(0.7)

                writer.writerow(
                    [
                        song_id,
                        diff_dict[chart["ratingClass"]],
                        chart_level,
                        round(chart_constant, 1),
                        chart["chartDesigner"],
                    ]
                )
