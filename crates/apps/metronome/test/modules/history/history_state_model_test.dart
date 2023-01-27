import 'package:flutter_test/flutter_test.dart';
import 'package:metronome/modules/history/session_entity.dart';
import 'package:metronome/modules/state/history_state_model.dart';

void main() {
  test("startOfDay returns the start of the day", () {
    final now = DateTime(
      2021,
      1,
      1,
      12,
    );
    final result = ChartTransformer.startOfDate(date: now);

    expect(
      result,
      equals(
        DateTime(
          2021,
        ),
      ),
    );
  });

  test(
      "startOfDay returns the start of the week if resolution is set to weekly",
      () {
    final now = DateTime(
      2023,
      1,
      26,
      12,
    );
    final result = ChartTransformer.startOfDate(
      date: now,
      resolution: HistoryResolution.weeks,
    );

    expect(
      result,
      equals(
        DateTime(2023, 1, 23),
      ),
    );
  });

  test("startOfDay trims out date time", () {
    final now = DateTime(2017, 9, 5, 17, 30, 15);
    final result = ChartTransformer.startOfDate(date: now);

    expect(
      result,
      equals(
        DateTime(2017, 9, 5),
      ),
    );
  });

  test(
      "preprocessPoints returns a list of 7 days starting at today if resolution is set to days",
      () {
    final data = [
      DailyPracticeTime.from(
        durationMs: 10000,
        timestampMs: DateTime(2017, 9, 7, 17, 30).millisecondsSinceEpoch,
      ),
      DailyPracticeTime.from(
        durationMs: 15000,
        timestampMs: DateTime(2017, 9, 5, 10, 30).millisecondsSinceEpoch,
      ),
    ];
    final DateTime now = DateTime(
      2017,
      9,
      9,
      12,
    );
    final result = ChartTransformer.preprocessPoints(
      now,
      data,
      HistoryResolution.days,
    );

    expect(result.length, equals(7));
    expect(
      result.map(
        (e) =>
            [e.durationMs, DateTime.fromMillisecondsSinceEpoch(e.timestampMs)],
      ),
      equals(
        [
          DailyPracticeTime.from(
            durationMs: 0,
            timestampMs: DateTime(2017, 9, 3).millisecondsSinceEpoch,
          ),
          DailyPracticeTime.from(
            durationMs: 0,
            timestampMs: DateTime(2017, 9, 4).millisecondsSinceEpoch,
          ),
          DailyPracticeTime.from(
            durationMs: 15000,
            timestampMs: DateTime(2017, 9, 5).millisecondsSinceEpoch,
          ),
          DailyPracticeTime.from(
            durationMs: 0,
            timestampMs: DateTime(2017, 9, 6).millisecondsSinceEpoch,
          ),
          DailyPracticeTime.from(
            durationMs: 10000,
            timestampMs: DateTime(2017, 9, 7).millisecondsSinceEpoch,
          ),
          DailyPracticeTime.from(
            durationMs: 0,
            timestampMs: DateTime(2017, 9, 8).millisecondsSinceEpoch,
          ),
          DailyPracticeTime.from(
            durationMs: 0,
            timestampMs: DateTime(2017, 9, 9).millisecondsSinceEpoch,
          ),
        ].map(
          (e) => [
            e.durationMs,
            DateTime.fromMillisecondsSinceEpoch(e.timestampMs)
          ],
        ),
      ),
    );
  });
  test(
      "preprocessPoints returns a list of 7 weeks starting at this week if resolution is set to weeks",
      () {
    final data = [
      DailyPracticeTime.from(
        durationMs: 10000,
        timestampMs: DateTime(2023, 1, 25, 17, 30).millisecondsSinceEpoch,
      ),
      DailyPracticeTime.from(
        durationMs: 15000,
        timestampMs: DateTime(2023, 1, 23, 10, 30).millisecondsSinceEpoch,
      ),
      DailyPracticeTime.from(
        durationMs: 15000,
        timestampMs: DateTime(2023, 1, 18, 10, 30).millisecondsSinceEpoch,
      ),
    ];
    final DateTime now = DateTime(
      2023,
      1,
      28,
    );
    final result = ChartTransformer.preprocessPoints(
      now,
      data,
      HistoryResolution.weeks,
    );

    expect(result.length, equals(7));
    expect(
      result.map(
        (e) =>
            [e.durationMs, DateTime.fromMillisecondsSinceEpoch(e.timestampMs)],
      ),
      equals(
        [
          [0, DateTime(2022, 12, 12)],
          [0, DateTime(2022, 12, 19)],
          [0, DateTime(2022, 12, 26)],
          [0, DateTime(2023, 01, 02)],
          [0, DateTime(2023, 01, 09)],
          [15000, DateTime(2023, 01, 16)],
          [15000, DateTime(2023, 01, 23)]
        ],
      ),
    );
  });
}
