import re
from collections import defaultdict

ENGINE_NAME = "latest"
PGN_FILE    = "games.pgn"

win_reasons  = defaultdict(int)
loss_reasons = defaultdict(int)
draw_reasons = defaultdict(int)

with open(PGN_FILE) as f:
    content = f.read()

for game in re.split(r'\n(?=\[Event )', content):
    white  = re.search(r'\[White "(.+?)"\]',  game)
    black  = re.search(r'\[Black "(.+?)"\]',  game)
    result = re.search(r'\[Result "(.+?)"\]', game)

    if not (white and black and result):
        continue

    white, black, result = white.group(1), black.group(1), result.group(1)

    is_white = ENGINE_NAME in white
    is_black = ENGINE_NAME in black
    if not (is_white or is_black):
        continue

    comments = re.findall(r'\{([^}]+)\}', game)
    last_comment = comments[-1].strip() if comments else ""

    if re.search(r'white mates', last_comment, re.IGNORECASE):
        reason = "Checkmated by White"
    elif re.search(r'black mates', last_comment, re.IGNORECASE):
        reason = "Checkmated by Black"
    elif re.search(r'stalemate', last_comment, re.IGNORECASE):
        reason = "Stalemate"
    elif re.search(r'3-fold repetition|threefold', last_comment, re.IGNORECASE):
        reason = "3-fold repetition"
    elif re.search(r'fifty[ -]move', last_comment, re.IGNORECASE):
        reason = "50-move rule"
    elif re.search(r'insufficient material', last_comment, re.IGNORECASE):
        reason = "Insufficient material"
    elif re.search(r'time', last_comment, re.IGNORECASE):
        reason = "Time forfeit"
    elif re.search(r'illegal', last_comment, re.IGNORECASE):
        reason = "Illegal move"
    elif re.search(r'adjudication', last_comment, re.IGNORECASE):
        reason = "Adjudication"
    else:
        reason = f"Other ({last_comment[:40]})" if last_comment else "Unknown"

    if result == "1/2-1/2":
        draw_reasons[reason] += 1
    elif (result == "1-0" and is_white) or (result == "0-1" and is_black):
        win_reasons[reason] += 1
    elif (result == "0-1" and is_white) or (result == "1-0" and is_black):
        loss_reasons[reason] += 1

def show(label, d):
    total = sum(d.values())
    print(f"\n{label} — Total: {total}")
    if d:
        for r, n in sorted(d.items(), key=lambda x: -x[1]):
            print(f"  {r}: {n}")
    else:
        print("  (none)")

total_games = sum(win_reasons.values()) + sum(loss_reasons.values()) + sum(draw_reasons.values())

print(f"\n=== Results for '{ENGINE_NAME}' — {total_games} games ===")
show("WINS  by reason", win_reasons)
show("LOSSES by reason", loss_reasons)
show("DRAWS  by reason", draw_reasons)